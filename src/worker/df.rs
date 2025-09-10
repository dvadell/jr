use crate::types::Metric;
use sysinfo::Disks;

pub fn run(mut metric: Metric) -> Metric {
    let disks = Disks::new_with_refreshed_list();
    let path = metric.args.clone();

    for disk in &disks {
        if disk.mount_point().to_str() == Some(&path) {
            let total = disk.total_space() as f64;
            let available = disk.available_space() as f64;
            let used = total - available;
            let used_percent = (used / total) * 100.0;

            metric.graph_short_name = Some(metric.short_name.clone());
            metric.value = Some(used_percent);
            metric.graph_value = Some(used_percent as i64);
            metric.units = Some("%".to_string());
            metric.message = Some(format!(
                "{} has {:.2}% used space",
                path, used_percent
            ));
            return metric;
        }
    }

    metric.value = Some(0.0);
    metric.graph_value = Some(0);
    metric.message = Some(format!("Filesystem '{}' not found", path));
    metric
}
