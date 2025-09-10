use std::process::Command;
use std::fs;
use serde_json::Value;
use tempfile::NamedTempFile;

#[test]
fn test_min_max_value_angelweb_output() {
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let temp_file_path = temp_file.path().to_str().expect("Failed to get temporary file path").to_string();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--once")
        .arg("--name")
        .arg("test_metric")
        .arg("--min-value")
        .arg("10.5")
        .arg("--max-value")
        .arg("20.5")
        .arg("--worker")
        .arg("timethis")
        .arg("--")
        .arg("sleep 0.1")
        .env("JR_TEST_OUTPUT_FILE", &temp_file_path) // Set the environment variable
        .output()
        .expect("Failed to execute command");

    // Check if the command ran successfully
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Read the content of the temporary file
    let file_content = fs::read_to_string(&temp_file_path).expect("Failed to read temporary file");
    let json_payload: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON from file content");

    assert_eq!(json_payload["min_value"].as_f64(), Some(10.5));
    assert_eq!(json_payload["max_value"].as_f64(), Some(20.5));
    assert_eq!(json_payload["short_name"].as_str(), Some("test_metric"));

    // The temporary file will be automatically deleted when `temp_file` goes out of scope
}

#[test]
fn test_graph_type_angelweb_output() {
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let temp_file_path = temp_file.path().to_str().expect("Failed to get temporary file path").to_string();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--once")
        .arg("--name")
        .arg("test_graph_type_metric")
        .arg("--worker")
        .arg("timethis")
        .arg("--")
        .arg("sleep 0.1")
        .env("JR_TEST_OUTPUT_FILE", &temp_file_path) // Set the environment variable
        .output()
        .expect("Failed to execute command");

    // Check if the command ran successfully
    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Read the content of the temporary file
    let file_content = fs::read_to_string(&temp_file_path).expect("Failed to read temporary file");
    let json_payload: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON from file content");

    assert_eq!(json_payload["type"].as_str(), Some("time"));
    assert_eq!(json_payload["graph_type"].as_str(), Some("time"));
    assert_eq!(json_payload["short_name"].as_str(), Some("test_graph_type_metric"));
}

#[test]
fn test_status_error_angelweb_output() {
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let temp_file_path = temp_file.path().to_str().expect("Failed to get temporary file path").to_string();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--once")
        .arg("--name")
        .arg("test_status_error")
        .arg("--worker")
        .arg("check_url")
        .arg("--")
        .arg("http://nonexistent.url.fail")
        .env("JR_TEST_OUTPUT_FILE", &temp_file_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));

    let file_content = fs::read_to_string(&temp_file_path).expect("Failed to read temporary file");
    let json_payload: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON from file content");

    assert_eq!(json_payload["status"].as_str(), Some("error"));
    assert_eq!(json_payload["short_name"].as_str(), Some("test_status_error"));
}

#[test]
fn test_status_ok_angelweb_output() {
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let temp_file_path = temp_file.path().to_str().expect("Failed to get temporary file path").to_string();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--once")
        .arg("--name")
        .arg("test_status_ok")
        .arg("--worker")
        .arg("timethis")
        .arg("--")
        .arg("sleep 0.1")
        .env("JR_TEST_OUTPUT_FILE", &temp_file_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));

    let file_content = fs::read_to_string(&temp_file_path).expect("Failed to read temporary file");
    let json_payload: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON from file content");

    assert_eq!(json_payload["status"].as_str(), Some("ok"));
    assert_eq!(json_payload["short_name"].as_str(), Some("test_status_ok"));
}

#[test]
fn test_df_worker() {
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    let temp_file_path = temp_file.path().to_str().expect("Failed to get temporary file path").to_string();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--once")
        .arg("--name")
        .arg("test_df")
        .arg("--worker")
        .arg("df")
        .arg("--")
        .arg("/")
        .env("JR_TEST_OUTPUT_FILE", &temp_file_path)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed with stderr: {}", String::from_utf8_lossy(&output.stderr));

    let file_content = fs::read_to_string(&temp_file_path).expect("Failed to read temporary file");
    let json_payload: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON from file content");

    assert_eq!(json_payload["units"].as_str(), Some("%"));
    let value = json_payload["graph_value"].as_u64().unwrap();
    assert!(value <= 100);
}
