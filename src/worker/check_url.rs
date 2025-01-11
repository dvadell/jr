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

