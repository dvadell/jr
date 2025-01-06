use sysinfo::System;
use crate::types::Result;

pub fn run() -> Result {
    // Initialize the system info
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    // Get the load average
    let load_avg = System::load_average();

    // return load_avg.one
    return Result { value: load_avg.one, message: "Hey".to_string()}
}
