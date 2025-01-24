use clap::Parser;
use std::ffi::OsString;
use std::env;

use crate::types::{Config,Args};

pub fn parse_config() -> Vec<Config>  {
    // Initialize a vector to store Config structures
    let mut configs: Vec<Config> = Vec::new();
    let args = Args::parse();

    
    let every = match args.every {
        Some(every) => every,
        None => 9999999
    };
    
    // Convert Vec<OsString> to String
    let remaining_args_str = args.remaining_args.iter()
    .map(|x| OsString::into_string(x.clone()))
    .map(Result::unwrap)
    .collect::<Vec<_>>()
    .join(" ");
    
    println!("Arguments: {}", remaining_args_str);
    
    let name = match &args.name {
        Some(name) => name,
        None => &placeholder_name(&remaining_args_str)
    };
    
    if ! remaining_args_str.is_empty() {
        configs.push(Config {
            n: every as u64,
            once: args.once,
            function: "timethis".to_string(),
            args: remaining_args_str,
            short_name: name.to_string()
        });
    }
    configs
}

fn placeholder_name(remaining_args_str: &String) -> String {
    let cmd_name = match remaining_args_str.split_whitespace().next() {
        Some(name) => name.to_string(),
        None => "".to_string()
    };

    let hostname = match env::var("HOSTNAME") {
        Ok(hostname) => hostname.to_string(),
        Err(_) => "no_hostname".to_string()
    };

    format!("{}_{}", cmd_name , hostname)
}