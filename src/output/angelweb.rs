// An output plugin to push the measure to angelweb.
use std::env;
use serde_json::json;
use crate::types::{Config,WorkerResult};

pub fn run(result: WorkerResult, _config: Config) ->  Result<(), Box<dyn std::error::Error>>  {
    let angelweb_server = env::var("ANGELWEB_SERVER").unwrap_or_else(|_| "http://127.0.0.1:4000".to_string());

    // graph_value and graph_short_name
    let units = result.units.unwrap_or("".to_string());
    let value: u32 = result.graph_value.unwrap_or(0);
    let short_name = result.graph_short_name.ok_or("no_name")?;
    let graph_type = result.graph_type.unwrap_or("g".to_string());

    println!("Angelweb is at {}. Sending {:?}", angelweb_server, value);

    // Make the HTTP request
    let client = reqwest::blocking::Client::new();
    let res = client.post(format!("{}//api/v1/metric", angelweb_server))
        .json(&json!({"short_name": short_name.to_string(), "graph_value": value, "units": units, "reporter": "jr@mordor", "type": graph_type }))
        .send()?;

    if res.status().is_success() {
        Ok(())
    } else {
        println!("Something else happened. Status: {:?}", res.status());
        Ok(())
    }
}
