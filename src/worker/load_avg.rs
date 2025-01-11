use sysinfo::System;
use crate::types::WorkerResult;

pub fn run(args: Option<&str>) -> WorkerResult {
    // Initialize the system info
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    // Get the load average
    let load_avg = System::load_average();

    let hostname = args.unwrap_or("localhost");

    // return load_avg.one
    return WorkerResult { 
        value: load_avg.one * 100.0, 
        message: "Hey".to_string(),
        graph_value: Some((load_avg.one * 100.0) as u32),
        graph_short_name: Some(format!("load_avg {}", hostname)),
        ..Default::default()
    }
}
