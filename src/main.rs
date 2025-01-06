use sysinfo::System;
use std::time::Duration;
use std::collections::HashMap;

mod config;
use config::file as conf;

#[derive(Debug)]
pub struct Result {
    pub value: f64,
    pub message: String,
}

fn main() {
    let mut function_map: HashMap<String, fn() -> Result> = HashMap::new();

    let config = conf::parse_config();
    // Set an interval for checking load average
    let check_interval = Duration::from_secs(config.n);

    // This will be completely dynamic, plug-in based
    function_map.insert("run".to_string(), run);

    loop {
        if let Some(func) = function_map.get(&config.function) {
            out( func() );
        }
        println!("{}", config.message);
        
        // Wait for the next interval
        std::thread::sleep(check_interval);
    }
}

fn run() -> Result {
    // Initialize the system info
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    // Get the load average
    let load_avg = System::load_average();

    // return load_avg.one
    return Result { value: load_avg.one, message: "Hey".to_string()}
}

fn out(result: Result) {
    println!("Value: {:.2}, message: {}", result.value, result.message);
}


