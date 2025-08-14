use std::time::Instant;

use crate::types::Metric;

pub fn run(mut metric: Metric) -> Metric {
    let url = match metric.args.trim().is_empty() {
        true => "https://lwn.net",
        false => metric.args.trim()
    };
    println!("Checking {}", url.to_string());

    // Start the timer
    let start_time = Instant::now();

    // Make the HTTP request
    match reqwest::blocking::get(url) {
        Ok(response) => {
            // Calculate the time taken
            let duration = start_time.elapsed().as_millis() as f64;
            
            // Check the response status code
            if response.status().is_success() {
                metric.value = Some(duration);
                metric.units = Some("ms".to_string());
                metric.message = Some("Success".to_string());
                metric.graph_value = Some(duration as u32);
            } else {
                metric.value = Some(duration);
                metric.units = Some("ms".to_string());
                metric.message = Some(format!("HTTP error: {}", response.status()));
                metric.graph_value = Some(duration as u32);
            }
        },
        Err(_e) => {
            metric.value = Some(0.0);
            metric.units = Some("ms".to_string());
            metric.message = Some("ERROR".to_string());
        }
    }
    metric
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Metric;

    #[test]
    fn test_check_url_success() {
        let metric = Metric {
            args: "https://www.google.com".to_string(),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.message, Some("Success".to_string()));
        assert!(result.value.unwrap() > 0.0);
    }

    #[test]
    fn test_check_url_failure() {
        let metric = Metric {
            args: "https://nonexistent.url.fail".to_string(),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.message, Some("ERROR".to_string()));
        assert_eq!(result.value, Some(0.0));
    }
}

