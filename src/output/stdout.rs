use crate::types::Metric;

pub fn run(metric: &Metric) {
    let mut output = format!("Value: {:.2} {:?}, message: {}", metric.value.unwrap_or_default(), metric.units.as_deref().unwrap_or_default(), metric.message.as_deref().unwrap_or_default());

    if let Some(min_value) = metric.min_value {
        output.push_str(&format!(", min: {}", min_value));
    }

    if let Some(max_value) = metric.max_value {
        output.push_str(&format!(", max: {}", max_value));
    }

    if let Some(graph_type) = &metric.graph_type {
        output.push_str(&format!(", graph_type: {}", graph_type));
    }

    println!("{}", output);
}
