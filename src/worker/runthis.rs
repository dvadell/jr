use crate::types::Metric;
use std::io::{self, Read};
use std::process::{Command, Stdio};
use std::str::FromStr;

pub fn run(mut metric: Metric) -> Metric {
    let command = metric.args.as_str();

    match run_command(command) {
        Ok(output) => {
            // try to parse output as int or float.
            let value = match parse_output(output) {
                Ok(Number::Integer(val)) => val,
                Ok(Number::Float(val)) => val as i32,
                Err(_e) => 0,
            };

            metric.value = Some(value as f64);
            metric.units = None;
            metric.message = Some("OK".to_string());
            metric.graph_value = Some(value as i64);
            metric.graph_short_name = Some(metric.short_name.clone());
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            metric.value = Some(-1.0);
            metric.units = None;
            metric.message = Some("Failed to execute command".to_string());
            metric.graph_value = Some(-1);
            metric.graph_short_name = Some(metric.short_name.clone());
            metric.status = "error".to_string();
        }
    }
    metric
}

fn run_command(command: &str) -> io::Result<String> {
    // Split the command into parts
    let args: Vec<&str> = command.split_whitespace().collect();

    // Create a new Command instance with the first argument as the program name and the rest as arguments
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty command"));
    }

    let cmd = &args[0];
    let args = &args[1..];

    // Spawn the command with standard input set to ignore and capture output and error
    let mut child = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Read stdout and stderr into buffers
    let mut stdout = String::new();
    let mut stderr = String::new();

    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout)?;
    }

    if let Some(mut err) = child.stderr.take() {
        err.read_to_string(&mut stderr)?;
    }

    // Wait for the command to finish and get the exit status
    let status = child.wait()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Command failed with status: {}. Error output: {}",
                status, stderr
            ),
        ));
    }

    // Return the captured standard output
    Ok(stdout)
}

fn parse_output(output: String) -> Result<Number, String> {
    match i32::from_str(output.trim()) {
        Ok(int_val) => return Ok(Number::Integer(int_val)),
        Err(_) => (),
    }

    match f64::from_str(output.trim()) {
        Ok(float_val) => return Ok(Number::Float(float_val)),
        Err(_) => (),
    }

    Err(format!(
        "Could not parse output as integer or float: {}",
        output
    ))
}

enum Number {
    Integer(i32),
    Float(f64),
}
