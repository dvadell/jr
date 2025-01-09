use std::time::Instant;

use crate::types::WorkerResult;

pub fn run(url: Option<&str>) -> WorkerResult {
    let url = url.unwrap_or("https://lwn.net");
    println!("Checking {}", url.to_string());

    // Start the timer
    let start_time = Instant::now();

    // Make the HTTP request
    match reqwest::blocking::get(url) {
        Ok(response) => {
            // Calculate the time taken
            let duration = start_time.elapsed().as_secs_f64();
            
            // Check the response status code
            if response.status().is_success() {
                WorkerResult {
                    value: duration,
                    message: "Success".to_string(),
                }
            } else {
                WorkerResult {
                    value: duration,
                    message: format!("HTTP error: {}", response.status()),
                }
            }
        },
        Err(_e) => {
            WorkerResult {
                value: 0.0,
                message: "ERROR".to_string(),
            }
        }
    }
}

