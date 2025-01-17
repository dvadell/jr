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

mod worker;
use worker::check_url as check_url;
use worker::load_avg as load_avg;
use worker::timethis as timethis;

mod types;
use crate::types::{WorkerResult,Args,Config};

use clap::Parser;



fn main() {
    let args = Args::parse();

    if let Some(every) = args.every {
        println!("Every option is set to: {}", every);
    } else {
        println!("The --every option was not provided.");
    }

    match &args.name {
        Some(name) => println!("Name option is: {}", name),
        None => println!("The --name option was not provided."),
    }

    let mut function_map: HashMap<String, fn(Config) -> WorkerResult> = HashMap::new();

    let mut configs = conf::parse_config();
    configs.extend_from_slice(&cmdline::parse_config());
    if configs.len() == 0 {
        eprintln!("Nothing to do");
        exit(1);
    }

    let now = Instant::now();

    // This will be completely dynamic, plug-in based
    function_map.insert("check_url".to_string(), check_url::run);
    function_map.insert("load_avg".to_string(), load_avg::run);
    function_map.insert("timethis".to_string(), timethis::run);

    loop {
        let start_time = now.elapsed().as_millis();
        let iteration = now.elapsed().as_secs();  // increments per second

        for config in &configs {
            if let Some(func) = function_map.get(&config.function) {
                // Only run every config.n seconds
                if iteration % config.n == 0 {
                    let result = func(config.clone());
                    let _ = out::run( result.clone(), config.clone() );
                    let _ = graphite::run( result.clone(), config.clone() );
                }
            }
        }

        // Wait for the next interval
        println!("Run took {}us", now.elapsed().as_millis() - start_time);
        let elapsed_nanos = now.elapsed().as_nanos();
        sleep(Duration::new(0, 1_000_000_000 - (elapsed_nanos % 1_000_000_000) as u32 ));

        if args.once {
            break;
        }
    }
}




