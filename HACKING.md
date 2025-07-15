# Hacking jr

This document provides a more detailed look at the internals of `jr`, with a focus on how to create new plugins.

## Project Structure

- `src/main.rs`: The main entry point of the application.
- `src/types.rs`: Defines the core data structures used throughout the application, such as `Config` and `WorkerResult`.
- `src/config/`: Handles parsing of the `jr.conf` file and command-line arguments.
- `src/worker/`: Contains the worker plugins.
- `src/output/`: Contains the output plugins.

## Core Data Structures

- **`Config`**: This structure holds the configuration for a single test, as parsed from a line in `jr.conf` or from command-line arguments.

  ```rust
  #[derive(Debug, Clone, Default)]
  pub struct Config {
      pub n: u64,            // The interval in seconds
      pub once: bool,          // Whether to run only once
      pub function: String,  // The name of the worker function to call
      pub args: String,        // The arguments to pass to the worker function
      pub short_name: String // The name of the test
  }
  ```

- **`WorkerResult`**: This structure is returned by worker plugins and contains the result of a test.

  ```rust
  #[derive(Debug, Clone)]
  pub struct WorkerResult {
      pub value: f64,
      pub message: String,
      pub graph_value: Option<u32>,
      pub graph_type: Option<String>,
      pub graph_name: Option<String>,
      pub graph_short_name: Option<String>
  }
  ```

## Creating a Worker Plugin

1.  **Create a new file** in the `src/worker/` directory (e.g., `src/worker/my_plugin.rs`).
2.  **Define a `run` function** in your new file with the following signature:

    ```rust
    use crate::types::{Config, WorkerResult};

    pub fn run(config: Config) -> WorkerResult {
        // Your plugin logic here
        WorkerResult {
            value: 1.0, // 1.0 for success, 0.0 for failure
            message: "Test completed successfully".to_string(),
            // ... other fields
        }
    }
    ```

3.  **Add your new module** to `src/worker/mod.rs`:

    ```rust
    pub mod my_plugin;
    ```

4.  **Register your plugin** in `src/main.rs` by adding it to the `function_map`:

    ```rust
    function_map.insert("my_plugin".to_string(), my_plugin::run);
    ```

## Creating an Output Plugin

1.  **Create a new file** in the `src/output/` directory (e.g., `src/output/my_output.rs`).
2.  **Define a `run` function** in your new file with the following signature:

    ```rust
    use crate::types::{Config, WorkerResult};

    pub fn run(result: WorkerResult, config: Config) -> Result<(), String> {
        // Your output logic here
        println!("Test '{}' result: {}", config.short_name, result.message);
        Ok(())
    }
    ```

3.  **Add your new module** to `src/output/mod.rs`:

    ```rust
    pub mod my_output;
    ```

4.  **Call your output plugin** in `src/main.rs` after a worker plugin has been executed:

    ```rust
    let result = func(config.clone());
    let _ = my_output::run(result.clone(), config.clone());
    ```
