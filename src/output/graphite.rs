use std::net::UdpSocket;
use crate::types::{WorkerResult,Config};

pub fn run(result: WorkerResult, config: Config) ->  Result<(), Box<dyn std::error::Error>>  {
    let ip_address = "127.0.0.1";

    // Create a UDP socket bound to a random local port
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Format the string according to the specified pattern
    let formatted_data = format!("jr.{}:{}|g", config.function, result.value as u32); // Graphite gauge needs int
    println!("{}", formatted_data);
    let data = formatted_data.into_bytes();
    
    // Send the data to the specified address on port 8125
    socket.send_to(&data, format!("{}:8125", ip_address))?;
    
    Ok(())
}