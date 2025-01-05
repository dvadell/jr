use std::fs;
use std::process::exit;

#[derive(Debug)]
pub struct Config {
    pub n: u64,
    pub message: String,
}

pub fn parse_config() -> Config {
    // Read the configuration file
    let config = fs::read_to_string("jr.conf").expect("Failed to read config file");
    
    // Parse the configuration into variables
    let n: u64 = 0;
    let message = String::new();

    // Parse the configuration into variables
    let mut parts = config.trim().splitn(2, "::");
    if let (Some(n_str), Some(message)) = (parts.next(), parts.next()) {
        if let Ok(n) = n_str.parse::<u64>() {
            return Config { n, message: message.to_string() };
        }
    }
    
    // Check if N is valid
    if n == 0 {
        eprintln!("Invalid N value in config file");
        exit(1);
    }
    
    Config { n, message }
}