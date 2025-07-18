use std::process::Command;
use std::io;
use std::time::Instant;
use crate::types::{Config,WorkerResult};

pub fn run(config: Config) -> WorkerResult {
    let command = config.args.as_str();

    let start = Instant::now();

    // time and execute
    match run_command(command) {
        Ok(_) => {
            println!("Command executed successfully.");
            return WorkerResult { 
                value: start.elapsed().as_millis() as f64,
                units: Some("ms".to_string()),
                message: "OK".to_string(),
                graph_value: Some((start.elapsed().as_millis()) as u32),
                graph_short_name: Some(config.short_name),
                ..Default::default()
            };

        },
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            return WorkerResult { 
                value: -1.0,
                units: Some("ms".to_string()),
                message: "Failed to execute command".to_string(),
                graph_value: Some(0 as u32),
                graph_short_name: Some(config.short_name),
                ..Default::default()
            };
        },
    }
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
    use crate::types::Config;

    #[test]
    fn test_timethis_success() {
        let config = Config {
            args: "echo hello".to_string(),
            short_name: "test".to_string(),
            ..Default::default()
        };
        let result = run(config);
        assert_eq!(result.message, "OK");
        assert!(result.value >= 0.0);
    }

    #[test]
    fn test_timethis_failure() {
        let config = Config {
            args: "nonexistentcommand".to_string(),
            short_name: "test".to_string(),
            ..Default::default()
        };
        let result = run(config);
        assert_eq!(result.message, "Failed to execute command");
        assert_eq!(result.value, -1.0);
    }
}
