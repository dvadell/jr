use std::env;
use std::net::UdpSocket;
use crate::types::Metric;

pub fn run(metric: &Metric) ->  Result<(), Box<dyn std::error::Error>>  {
    let ip_address = match env::var("GRAPHITE_SERVER") {
        Ok(value) => value,
        Err(_) => "127.0.0.1".to_string()
    };

    // Create a UDP socket bound to a random local port
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let graph_short_name = metric.graph_short_name.as_deref().unwrap_or(&metric.function);
    let value = metric.graph_value.unwrap_or(metric.value.unwrap_or_default() as i64);

    let metric_type = if metric.graph_type.as_deref() == Some("time") {
        "ms" // Assuming units are always milliseconds for time
    } else {
        "g" // Default to gauge
    };

    // Format the string according to the specified pattern
    let formatted_data = format!("jr.{}.{}:{}|{}", graph_short_name, metric.status, value, metric_type);
    println!("{}", formatted_data);
    let data = formatted_data.into_bytes();
    
    // Send the data to the specified address on port 8125
    socket.send_to(&data, format!("{}:8125", ip_address))?;
    
    Ok(())
}