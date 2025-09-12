use std::collections::HashMap;
use std::env;

use reqwest::blocking::Client;
use serde_json::Value;

use crate::types::Metric;

fn parse_args(args: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for part in args.split_whitespace() {
        if let Some((key, value)) = part.split_once('=') {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

/// A worker plugin that queries an API and extracts a numeric value from its JSON response
/// using a jq-like path traversal.
///
/// Example configuration line in `jr.conf`:
/// `dolarapi_blue_venta::300::query_api::url=https://dolarapi.com/v1/dolares/blue jq=.venta`
pub fn run(mut metric: Metric) -> Metric {
    let args = parse_args(&metric.args);

    let debug_enabled = env::var("DEBUG").is_ok_and(|v| v == "1");

    let url = match args.get("url") {
        Some(url) => url,
        None => {
            metric.status = "error".to_string();
            metric.message = Some("url is a mandatory argument".to_string());
            metric.value = Some(-1.0);
            return metric;
        }
    };

    let method = args
        .get("method")
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "GET".to_string());
    let jq_path = args.get("jq");

    metric.graph_short_name = Some(metric.short_name.clone());

    let client = Client::new();
    let request_builder = match method.as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        _ => {
            metric.status = "error".to_string();
            metric.message = Some(format!("Unsupported HTTP method: {}", method));
            metric.value = Some(-1.0);
            return metric;
        }
    };

    match request_builder.send() {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Value>() {
                    Ok(json) => {
                        if debug_enabled {
                            println!(
                                "DEBUG: Full API JSON response:\n{}",
                                serde_json::to_string_pretty(&json).unwrap_or_default()
                            );
                        }

                        if let Some(path) = jq_path {
                            let mut current_value = &json;
                            for key in path.trim_start_matches('.').split('.') {
                                if let Some(next_value) = current_value.get(key) {
                                    current_value = next_value;
                                } else {
                                    metric.status = "error".to_string();
                                    metric.message =
                                        Some(format!("jq path error: key '{}' not found", key));
                                    metric.value = Some(-1.0);
                                    return metric;
                                }
                            }
                            if let Some(v) = current_value.as_f64() {
                                metric.value = Some(v);
                                metric.graph_value = Some(v as i64); // Set graph_value here
                                metric.status = "ok".to_string();
                                if debug_enabled {
                                    println!("DEBUG: JQ traversal output (numeric): {}", v);
                                }
                            } else {
                                metric.status = "error".to_string();
                                metric.message = Some(format!(
                                    "jq path error: value at '{}' is not a number",
                                    path
                                ));
                                metric.value = Some(-1.0);
                                if debug_enabled {
                                    println!(
                                        "DEBUG: JQ traversal output (non-numeric): {}",
                                        current_value
                                    );
                                }
                            }
                        } else {
                            metric.status = "error".to_string();
                            metric.message =
                                Some("jq path is mandatory for this worker".to_string());
                            metric.value = Some(-1.0);
                        }
                    }
                    Err(e) => {
                        metric.status = "error".to_string();
                        metric.message = Some(format!("JSON parsing error: {}", e));
                        metric.value = Some(-1.0);
                    }
                }
            } else {
                metric.status = "error".to_string();
                metric.message = Some(format!("HTTP error: {}", response.status()));
                metric.value = Some(-1.0);
            }
        }
        Err(e) => {
            metric.status = "error".to_string();
            metric.message = Some(format!("Request error: {}", e));
            metric.value = Some(-1.0);
        }
    }

    metric
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Metric;
    use httptest::{matchers::*, responders::*, Expectation, Server};

    #[test]
    fn test_query_api_missing_url() {
        let metric = Metric {
            args: "method=GET".to_string(),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.status, "error");
        assert_eq!(
            result.message,
            Some("url is a mandatory argument".to_string())
        );
        assert_eq!(result.value, Some(-1.0));
    }

    #[test]
    fn test_query_api_missing_jq() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/"))
                .respond_with(json_encoded(serde_json::json!({"status": "ok"}))),
        );

        let metric = Metric {
            args: format!("url={}", server.url("/")),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.status, "error");
        assert_eq!(
            result.message,
            Some("jq path is mandatory for this worker".to_string())
        );
        assert_eq!(result.value, Some(-1.0));
    }

    #[test]
    fn test_query_api_success_with_jq() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/"))
                .respond_with(json_encoded(serde_json::json!({"data": {"value": 42}}))),
        );

        let metric = Metric {
            args: format!("url={} jq=.data.value", server.url("/")),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.status, "ok");
        assert_eq!(result.value, Some(42.0));
        assert_eq!(result.graph_value, Some(42)); // New assertion
    }

    #[test]
    fn test_query_api_jq_path_error() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/"))
                .respond_with(json_encoded(serde_json::json!({"data": {"value": 42}}))),
        );

        let metric = Metric {
            args: format!("url={} jq=.data.nonexistent", server.url("/")),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.status, "error");
        assert_eq!(
            result.message,
            Some("jq path error: key 'nonexistent' not found".to_string())
        );
        assert_eq!(result.value, Some(-1.0));
    }

    #[test]
    fn test_query_api_jq_path_not_numeric() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/")).respond_with(json_encoded(
                serde_json::json!({"data": {"value": "hello"}}),
            )),
        );

        let metric = Metric {
            args: format!("url={} jq=.data.value", server.url("/")),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.status, "error");
        assert_eq!(
            result.message,
            Some("jq path error: value at '.data.value' is not a number".to_string())
        );
        assert_eq!(result.value, Some(-1.0));
    }

    #[test]
    fn test_query_api_http_error() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/")).respond_with(status_code(404)),
        );

        let metric = Metric {
            args: format!("url={} jq=.status", server.url("/")),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.status, "error");
        assert_eq!(
            result.message,
            Some("HTTP error: 404 Not Found".to_string())
        );
        assert_eq!(result.value, Some(-1.0));
    }

    #[test]
    fn test_query_api_request_error() {
        let metric = Metric {
            args: "url=http://localhost:12345 jq=.status".to_string(),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.status, "error");
        assert!(result.message.unwrap().contains("Request error"));
        assert_eq!(result.value, Some(-1.0));
    }
}
