use clap::Parser;
use std::ffi::OsString;

use crate::types::{Config,Args};

pub fn parse_config() -> Vec<Config>  {
    let args = Args::parse();

    let name = match &args.name {
        Some(name) => name,
        None => "No name"
    };

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

    // Initialize a vector to store Config structures
    let mut configs: Vec<Config> = Vec::new();

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