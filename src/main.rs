use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::process::exit;

mod config;
use config::file as conf;
use config::cmdline;

mod output;
use output::stdout as out;
use output::graphite;
use output::angelweb;

mod worker;
use worker::check_url as check_url;
use worker::load_avg as load_avg;
use worker::timethis as timethis;
use worker::runthis as runthis;

mod types;
use crate::types::Metric;


fn main() {
    let mut function_map: HashMap<String, fn(Metric) -> Metric> = HashMap::new();

    let mut configs = conf::parse_config();
    configs.extend_from_slice(&cmdline::parse_config());
    if configs.len() == 0 {
        eprintln!("No configuration found. Please provide command-line arguments or a configuration file. Use `jr --help` for more information.");
        exit(1);
    }

    let now = Instant::now();

    // This will be completely dynamic, plug-in based
    function_map.insert("check_url".to_string(), check_url::run);
    function_map.insert("load_avg".to_string(), load_avg::run);
    function_map.insert("timethis".to_string(), timethis::run);
    function_map.insert("runthis".to_string(), runthis::run);

    loop {
        let start_time = now.elapsed().as_millis();
        let iteration = now.elapsed().as_secs();  // increments per second

        for metric in &mut configs {
            if let Some(func) = function_map.get(&metric.function) {
                // Only run every metric.n seconds
                if iteration % metric.n == 0 {
                    let result_metric = func(metric.clone());
                    *metric = result_metric;
                    let _ = out::run( metric );
                    let _ = graphite::run( metric );
                    let _ = angelweb::run( metric );
                }
            }
        }

        // Wait for the next interval
        println!("Run took {}us", now.elapsed().as_millis() - start_time);
        let elapsed_nanos = now.elapsed().as_nanos();
        sleep(Duration::new(0, 1_000_000_000 - (elapsed_nanos % 1_000_000_000) as u32 ));

        if configs.iter().any(|c| c.once) {
            break;
        }
    }
}




