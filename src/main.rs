use sysinfo::System;
use std::time::Duration;

mod config;
use config::file as conf;

fn main() {
    let config = conf::parse_config();
    // Set an interval for checking load average
    let check_interval = Duration::from_secs(config.n);

    loop {
        run();
        println!("{}", config.message);
        
        // Wait for the next interval
        std::thread::sleep(check_interval);
    }
}

fn run() {
    // Initialize the system info
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    // Get the load average
    let load_avg = System::load_average();
    println!("Load Average: {:.2}, {:.2}, {:.2}", load_avg.one, load_avg.five, load_avg.fifteen);
}


