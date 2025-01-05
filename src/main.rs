use std::time::{Duration, Instant};
use sysinfo::System;

fn main() {
    // Initialize the system info
    let mut system = System::new_all();

    // Set an interval for checking load average
    let check_interval = Duration::from_secs(5);

    // Get the start time
    let start_time = Instant::now();

    loop {
        // Refresh system information
        system.refresh_all();

        // Calculate the elapsed time since the start of the program
        let elapsed = start_time.elapsed();

        // Check if the elapsed time is a multiple of the check interval
        if elapsed.as_secs() % check_interval.as_secs() == 0 {
            // Get the load average
            let load_avg = System::load_average();
            println!("Load Average: {:.2}, {:.2}, {:.2}", load_avg.one, load_avg.five, load_avg.fifteen);
        }

        // Wait for the next interval
        std::thread::sleep(check_interval);
    }
}
