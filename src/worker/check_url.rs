use std::time::Instant;

use crate::types::Result;

pub fn run(url: Option<&str>) -> Result {
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
                Result {
                    value: duration,
                    message: "Success".to_string(),
                }
            } else {
                Result {
                    value: duration,
                    message: format!("HTTP error: {}", response.status()),
                }
            }
        },
        Err(e) => {
            Result {
                value: 0.0,
                message: "ERROR".to_string(),
            }
        }
    }
}

