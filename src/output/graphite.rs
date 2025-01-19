use std::env;
use std::net::UdpSocket;
use crate::types::{WorkerResult,Config};

pub fn run(result: WorkerResult, config: Config) ->  Result<(), Box<dyn std::error::Error>>  {
    let ip_address = match env::var("GRAPHITE_SERVER") {
        Ok(value) => value,
        Err(_) => "127.0.0.1".to_string()
    };

    // Create a UDP socket bound to a random local port
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    let graph_short_name = result.graph_short_name.unwrap_or(config.function);
    let value = result.graph_value.unwrap_or(result.value as u32);

    // Format the string according to the specified pattern
    let formatted_data = format!("jr.{}:{}|g", graph_short_name, value); // Graphite gauge needs int
    println!("{}", formatted_data);
    let data = formatted_data.into_bytes();
    
    // Send the data to the specified address on port 8125
    socket.send_to(&data, format!("{}:8125", ip_address))?;
    
    Ok(())
}