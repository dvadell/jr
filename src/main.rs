use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread::sleep;

mod config;
use config::file as conf;

mod output;
use output::stdout as out;
use output::graphite;

mod worker;
use worker::check_url as check_url;
use worker::load_avg as load_avg;

mod types;
use crate::types::WorkerResult;

fn main() {
    let mut function_map: HashMap<String, fn(Option<&str>) -> WorkerResult> = HashMap::new();

    let configs = conf::parse_config();
    let now = Instant::now();

    // This will be completely dynamic, plug-in based
    function_map.insert("check_url".to_string(), check_url::run);
    function_map.insert("load_avg".to_string(), load_avg::run);

    loop {
        let start_time = now.elapsed().as_millis();
        let iteration = now.elapsed().as_secs();  // increments per second

        for config in &configs {
            if let Some(func) = function_map.get(&config.function) {
                // Only run every config.n seconds
                if iteration % config.n == 0 {
                    let result = func(Some(&config.args));
                    let _ = out::run( result.clone() );
                    let _ = graphite::run( result.clone() );
                }
            }
        }

        // Wait for the next interval
        println!("Run took {}us", now.elapsed().as_millis() - start_time);
        let elapsed_nanos = now.elapsed().as_nanos();
        sleep(Duration::new(0, 1_000_000_000 - (elapsed_nanos % 1_000_000_000) as u32 ));
    }
}




