// An output plugin to push the measure to angelweb.
use std::env;
use serde_json::json;
use crate::types::Metric;

pub fn run(metric: &Metric) ->  Result<(), Box<dyn std::error::Error>>  {
    let angelweb_server = env::var("ANGELWEB_SERVER").unwrap_or_else(|_| "http://127.0.0.1:4000".to_string());

    // graph_value and graph_short_name
    let units = metric.units.as_deref().unwrap_or("");
    let group = &metric.group;
    let value: u32 = metric.graph_value.unwrap_or(0);
    let short_name = metric.graph_short_name.as_deref().ok_or("no_name")?;
    let graph_type = metric.graph_type.as_deref().unwrap_or("g");

    println!("Angelweb is at {}. Sending {:?}", angelweb_server, value);

    // Make the HTTP request
    let client = reqwest::blocking::Client::new();
    let payload = json!({
        "short_name": short_name,
        "graph_value": value,
        "units": units,
        "group": group,
        "reporter": "jr@mordor",
        "type": graph_type,
        "min_value": metric.min_value,
        "max_value": metric.max_value
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
