use sysinfo::System;
use crate::types::Metric;

pub fn run(mut metric: Metric) -> Metric {
    // Initialize the system info
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    // Get the load average
    let load_avg = System::load_average();

    let hostname = match metric.args.trim().is_empty() {
        true => "localhost",
        false => metric.args.trim()
    };

    metric.value = Some(load_avg.one * 100.0);
    metric.message = Some("Hey".to_string());
    metric.graph_value = Some((load_avg.one * 100.0) as i64);
    metric.graph_short_name = Some(format!("load_avg_{}", hostname));
    metric
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Metric;

    #[test]
    fn test_load_avg() {
        let metric = Metric {
            args: "localhost".to_string(),
            ..Default::default()
        };
        let result = run(metric);
        assert!(result.value.unwrap() >= 0.0);
    }
}
