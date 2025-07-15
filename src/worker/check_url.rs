use std::time::Instant;

use crate::types::{Config,WorkerResult};

pub fn run(config: Config) -> WorkerResult {
    let url = match config.args.trim().is_empty() {
        true => "https://lwn.net",
        false => config.args.trim()
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
                WorkerResult {
                    value: duration,
                    message: "Success".to_string(),
                    graph_value: Some(duration as u32),
                    ..Default::default()
                }
            } else {
                WorkerResult {
                    value: duration,
                    message: format!("HTTP error: {}", response.status()),
                    graph_value: Some(duration as u32),
                    ..Default::default()
                }
            }
        },
        Err(_e) => {
            WorkerResult {
                value: 0.0,
                message: "ERROR".to_string(),
                ..Default::default()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Config;

    #[test]
    fn test_check_url_success() {
        let config = Config {
            args: "https://www.google.com".to_string(),
            ..Default::default()
        };
        let result = run(config);
        assert_eq!(result.message, "Success");
        assert!(result.value > 0.0);
    }

    #[test]
    fn test_check_url_failure() {
        let config = Config {
            args: "https://nonexistent.url.fail".to_string(),
            ..Default::default()
        };
        let result = run(config);
        assert_eq!(result.message, "ERROR");
        assert_eq!(result.value, 0.0);
    }
}

