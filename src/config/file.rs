use std::fs;
use std::process::exit;

#[derive(Debug)]
pub struct Config {
    pub n: u64,
    pub message: String,
    pub function: String,
}

pub fn parse_config() -> Config {
    // Read the configuration file
    let config = fs::read_to_string("jr.conf").expect("Failed to read config file");
    
    // Parse the configuration into variables
    let mut parts = config.trim().splitn(3, "::");
    
    if let (Some(n_str), Some(message), Some(function)) = (parts.next(), parts.next(), parts.next()) {
        if let Ok(n) = n_str.parse::<u64>() {
            if n == 0 {
                eprintln!("Invalid N value in config file");
                exit(1);
            }
            return Config {
                n,
                message: message.to_string(),
                function: function.to_string()
            };
        }
    }
    
    eprintln!("Failed to parse config file");
    exit(1);
}