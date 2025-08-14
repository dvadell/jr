use crate::types::Metric;

pub fn run(metric: &Metric) {
    println!("Value: {:.2} {:?}, message: {}", metric.value.unwrap_or_default(), metric.units.as_deref().unwrap_or_default(), metric.message.as_deref().unwrap_or_default());
}
