use std::time::Duration;
use std::collections::HashMap;

mod config;
use config::file as conf;

mod output;
use output::stdout as out;

mod worker;
use worker::check_url as check_url;
use worker::load_avg as load_avg;

mod types;
use crate::types::Result;

fn main() {
    let mut function_map: HashMap<String, fn(Option<&str>) -> Result> = HashMap::new();

    let config = conf::parse_config();
    // Set an interval for checking load average
    let check_interval = Duration::from_secs(config.n);

    // This will be completely dynamic, plug-in based
    function_map.insert("check_url".to_string(), check_url::run);
    function_map.insert("load_avg".to_string(), load_avg::run);

    loop {
        if let Some(func) = function_map.get(&config.function) {
            out::run( func(Some(&config.args)) );
        }
        println!("{}", config.function);
        
        // Wait for the next interval
        std::thread::sleep(check_interval);
    }
}




