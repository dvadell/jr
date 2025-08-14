# Hacking jr

This document provides a more detailed look at the internals of `jr`, with a focus on how to create new plugins.

## Project Structure

- `src/main.rs`: The main entry point of the application.
- `src/types.rs`: Defines the core data structures used throughout the application, such as `Metric`.
- `src/config/`: Handles parsing of the `jr.conf` file and command-line arguments.
- `src/worker/`: Contains the worker plugins.
- `src/output/`: Contains the output plugins.

## Core Data Structures

The core data structure is **`Metric`**. It holds the configuration for a single test, and is then updated by the worker with the test result.

  ```rust
  #[derive(Debug, Clone, Default)]
  pub struct Metric {
      // From Config
      pub n: u64,
      pub once: bool,
      pub function: String,
      pub group: String,
      pub args: String,
      pub short_name: String,

      // From WorkerResult
      pub value: Option<f64>,
      pub units: Option<String>,
      pub message: Option<String>,
      pub graph_value: Option<u32>,
      pub graph_type: Option<String>,
      pub graph_name: Option<String>,
      pub graph_short_name: Option<String>,
  }
  ```

## Creating a Worker Plugin

1.  **Create a new file** in the `src/worker/` directory (e.g., `src/worker/my_plugin.rs`).
2.  **Define a `run` function** in your new file with the following signature:

    ```rust
    use crate::types::Metric;

    pub fn run(mut metric: Metric) -> Metric {
        // Your plugin logic here
        metric.value = Some(1.0); // 1.0 for success, 0.0 for failure
        metric.message = Some("Test completed successfully".to_string());
        // ... other fields
        metric
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
    use crate::types::Metric;

    pub fn run(metric: &Metric) -> Result<(), String> {
        // Your output logic here
        println!("Test '{}' result: {}", metric.short_name, metric.message.as_deref().unwrap_or(""));
        Ok(())
    }
    ```

3.  **Add your new module** to `src/output/mod.rs`:

    ```rust
    pub mod my_output;
    ```

4.  **Call your output plugin** in `src/main.rs` after a worker plugin has been executed:

    ```rust
    let result_metric = func(metric.clone());
    *metric = result_metric;
    let _ = my_output::run(metric);
    ```
