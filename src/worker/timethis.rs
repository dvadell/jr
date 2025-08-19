use std::process::Command;
use std::io;
use std::time::Instant;
use crate::types::Metric;

pub fn run(mut metric: Metric) -> Metric {
    let command = metric.args.as_str();

    let start = Instant::now();

    // time and execute
    match run_command(command) {
        Ok(_) => {
            println!("Command executed successfully.");
            metric.value = Some(start.elapsed().as_millis() as f64);
            metric.units = Some("ms".to_string());
            metric.message = Some("OK".to_string());
            metric.graph_value = Some(start.elapsed().as_millis() as i64);
            metric.graph_type = Some("time".to_string());
            metric.graph_short_name = Some(metric.short_name.clone());

        },
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            metric.value = Some((start.elapsed().as_millis() as f64) * -1.0);
            metric.units = Some("ms".to_string());
            metric.message = Some("Failed to execute command".to_string());
            metric.graph_value = Some((start.elapsed().as_millis() as i64) * -1);
            metric.graph_short_name = Some(metric.short_name.clone());
        },
    }
    metric
}


fn run_command(command: &str) -> io::Result<()> {
    // Split the command into parts
    let args: Vec<&str> = command.split_whitespace().collect();

    // Create a new Command instance with the first argument as the program name and the rest as arguments
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty command"));
    }

    let cmd = &args[0];
    let args = &args[1..];

    // Spawn the command with standard input, output, and error set to ignore
    let mut child = Command::new(cmd)
        .args(args)
        .spawn()?;

    // Wait for the command to finish and get the exit status
    let status = child.wait()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed with status: {}", status),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Metric;

    #[test]
    fn test_timethis_success() {
        let metric = Metric {
            args: "echo hello".to_string(),
            short_name: "test".to_string(),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.message, Some("OK".to_string()));
        assert!(result.value.unwrap() >= 0.0);
    }

    #[test]
    fn test_timethis_failure() {
        let metric = Metric {
            args: "nonexistentcommand".to_string(),
            short_name: "test".to_string(),
            ..Default::default()
        };
        let result = run(metric);
        assert_eq!(result.message, Some("Failed to execute command".to_string()));
        assert_eq!(result.value, Some(-1.0));
    }
}
