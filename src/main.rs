use std::time::Duration;
use std::collections::HashMap;

mod config;
use config::file as conf;

mod output;
use output::stdout as out;

mod worker;
use worker::load_avg as work;

mod types;
use crate::types::Result;

fn main() {
    let mut function_map: HashMap<String, fn() -> Result> = HashMap::new();

    let config = conf::parse_config();
    // Set an interval for checking load average
    let check_interval = Duration::from_secs(config.n);

    // This will be completely dynamic, plug-in based
    function_map.insert("run".to_string(), work::run);

    loop {
        if let Some(func) = function_map.get(&config.function) {
            out::run( func() );
        }
        println!("{}", config.message);
        
        // Wait for the next interval
        std::thread::sleep(check_interval);
    }
}




