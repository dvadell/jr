use std::fs;
use std::process::exit;

use crate::types::Config;

pub fn parse_config() -> Config {
    // Read the configuration file
    let config = fs::read_to_string("jr.conf").expect("Failed to read config file");
    
    // Parse the configuration into variables
    let mut parts = config.trim().splitn(3, "::");
    
    if let (Some(n_str), Some(function), Some(args)) = (parts.next(), parts.next(), parts.next()) {
        if let Ok(n) = n_str.parse::<u64>() {
            if n == 0 {
                eprintln!("Invalid N value in config file");
                exit(1);
            }
            return Config {
                n,
                function: function.to_string(),
                args: args.to_string()
            };
        }
    }
    
    eprintln!("Failed to parse config file");
    exit(1);
}