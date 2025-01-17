use std::fs;

use crate::types::Config;

fn parse_line(line: &str) -> Option<Config> {
    // Trim any leading/trailing whitespace and split the line by "::" up to 3 parts
    let mut parts = line.trim().splitn(4, "::");
    
    if let (Some(short_name), Some(n_str), Some(function), Some(args)) = (parts.next(), parts.next(), parts.next(), parts.next()) {
        // Check for comment lines starting with # or //
        if n_str.starts_with('#') || n_str.starts_with("//") {
            return None;
        }
        
        // Parse the first part as an unsigned 64-bit integer (`n`)
        if let Ok(n) = n_str.parse::<u64>() {
            if n == 0 {
                eprintln!("Invalid N value in config file at line: {}", line);
                return None;
            }
            Some(Config {
                n,
                function: function.to_string(),
                args: args.to_string(),
                short_name: short_name.to_string(),
                ..Default::default()
            })
        } else {
            eprintln!("Failed to parse N value in config file at line: {}", line);
            None
        }
    } else if line.trim_start().starts_with('#') || line.trim_start().starts_with("//") {
        // Ignore comment lines starting with # or //
        None
    } else {
        eprintln!("Invalid format in config file at line: {}", line);
        None
    }
}

pub fn parse_config() -> Vec<Config> {
    // Initialize a vector to store Config structures
    let mut configs: Vec<Config> = Vec::new();
    
    // Read the configuration file. Return empty configs if no config file.
    let config = match fs::read_to_string("jr.conf") {
        Ok(content) => content,
        Err(_) => return configs
    };
    
    // Split the content into lines
    let lines: Vec<&str> = config.trim().lines().collect();
    
    // Iterate over each line and parse it into a Config structure
    for line in lines {
        if let Some(config) = parse_line(line) {
            configs.push(config);
        }
    }
    
    configs
}