use sysinfo::System;
use crate::types::WorkerResult;

pub fn run(_args: Option<&str>) -> WorkerResult {
    // Initialize the system info
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    // Get the load average
    let load_avg = System::load_average();

    // return load_avg.one
    return WorkerResult { value: load_avg.one * 100.0, message: "Hey".to_string()}
}
