use clap::Parser;
use std::ffi::OsString;
use std::env;

use crate::types::{Metric,Args};

pub fn parse_config() -> Vec<Metric>  {
    let args = env::args_os().collect::<Vec<_>>();
    parse_config_from_args(args)
}

pub fn parse_config_from_args(args: Vec<OsString>) -> Vec<Metric> {
    // Initialize a vector to store Config structures
    let mut configs: Vec<Metric> = Vec::new();
    let args = Args::parse_from(args);

    if args.version {
        println!("jr {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let worker = match args.worker {
        Some(every) => every,
        None => "timethis".to_string(),
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
    
    let name = match &args.name {
        Some(name) => name,
        None => &placeholder_name(&remaining_args_str)
    };
    
    configs.push(Metric {
        n: every as u64,
        once: args.once,
        function: worker,
        group: "".to_string(),
        args: remaining_args_str,
        short_name: name.to_string(),
        ..Default::default()
    });
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

#[cfg(test)]
mod tests {
    use crate::config::cmdline::parse_config_from_args;
    use std::ffi::OsString;

    #[test]
    fn test_worker_flag() {
        let args: Vec<OsString> = vec!["jr".into(), "--worker".into(), "test_worker".into()];
        let config = parse_config_from_args(args);
        assert_eq!(config.len(), 1);
        assert_eq!(config[0].function, "test_worker");
    }
}
