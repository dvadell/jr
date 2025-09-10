// An output plugin to push the measure to angelweb.
use std::env;
use std::fs;
use serde_json::json;
use crate::types::Metric;

pub fn run(metric: &Metric) ->  Result<(), Box<dyn std::error::Error>>  {
    // If JR_TEST_OUTPUT_FILE is set, write the JSON payload to the specified file
    if let Ok(output_file) = env::var("JR_TEST_OUTPUT_FILE") {
        let payload = json!({
            "short_name": metric.short_name,
            "graph_value": metric.graph_value.unwrap_or(0),
            "units": metric.units.as_deref().unwrap_or(""),
            "group": metric.group,
            "reporter": "jr@mordor",
            "type": metric.graph_type.as_deref().unwrap_or("g"),
            "graph_type": metric.graph_type.as_deref().unwrap_or(""),
            "min_value": metric.min_value,
            "max_value": metric.max_value,
            "every": if metric.once { -1 } else { metric.n as i64 },
            "status": metric.status
        });
        fs::write(output_file, serde_json::to_string_pretty(&payload)?)?;
        return Ok(());
    }

    let angelweb_server = env::var("ANGELWEB_SERVER").unwrap_or_else(|_| "http://127.0.0.1:4000".to_string());

    // graph_value and graph_short_name
    let units = metric.units.as_deref().unwrap_or("");
    let group = &metric.group;
    let value: i64 = metric.graph_value.unwrap_or(0);
    let short_name = match metric.graph_short_name.as_deref() {
        Some(name) => name,
        None => {
            eprintln!("Warning: metric '{}' is missing 'graph_short_name'. Skipping angelweb output.", metric.short_name);
            return Ok(());
        }
    };
    let graph_type = metric.graph_type.as_deref().unwrap_or("");

    println!("Angelweb is at {}. Sending {:?}", angelweb_server, value);

    // Make the HTTP request
    let client = reqwest::blocking::Client::new();
    let payload = json!({
        "short_name": short_name,
        "graph_value": value,
        "units": units,
        "group": group,
        "reporter": "jr@mordor",
        "graph_type": graph_type,
        "min_value": metric.min_value,
        "max_value": metric.max_value,
        "every": if metric.once { -1 } else { metric.n as i64 },
        "status": &metric.status
    });

    if env::var("DEBUG").unwrap_or_else(|_| "0".to_string()) == "1" {
        println!("JSON payload: {}", serde_json::to_string_pretty(&payload).unwrap());
    }

    let res = client.post(format!("{}//api/v1/metric", angelweb_server))
        .json(&payload)
        .send()?;

    if res.status().is_success() {
        Ok(())
    } else {
        println!("Something else happened. Status: {:?}", res.status());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Metric;
    use std::env;
    use std::fs;
    use serde_json::Value;
    use tempfile::NamedTempFile;

    #[test]
    fn test_run_with_every() {
        let output_file = NamedTempFile::new().unwrap();
        let output_file_path = output_file.path().to_str().unwrap();
        env::set_var("JR_TEST_OUTPUT_FILE", output_file_path);

        // Test with a specific 'every' value
        let metric1 = Metric {
            n: 60,
            once: false,
            ..Default::default()
        };
        run(&metric1).unwrap();
        let content = fs::read_to_string(output_file_path).unwrap();
        let json: Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["every"], 60);

        // Test with 'once' set to true
        let metric2 = Metric {
            once: true,
            ..Default::default()
        };
        run(&metric2).unwrap();
        let content = fs::read_to_string(output_file_path).unwrap();
        let json: Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["every"], -1);

        // Clean up
        env::remove_var("JR_TEST_OUTPUT_FILE");
    }
}