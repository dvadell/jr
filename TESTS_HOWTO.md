# How to Write Tests for New Plugins

This project uses unit tests for its plugins. Each plugin module should have its own test module within the same file.

## Steps to Write a Test for a New Plugin

1.  **Locate the Plugin Module:** Navigate to the `src/worker` directory and find the Rust file corresponding to your plugin (e.g., `src/worker/my_new_plugin.rs`).

2.  **Add a Test Module:** At the end of the plugin's `.rs` file, add a test module using the `#[cfg(test)]` attribute. This ensures the test code is only compiled when running tests.

    ```rust
    #[cfg(test)]
    mod tests {
        // Test code goes here
    }
    ```

3.  **Import Necessary Items:** Inside your `tests` module, import the necessary items from the parent module and any other crates.

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::types::{Config, WorkerResult};
        // Add any other necessary imports
    }
    ```

4.  **Write Test Functions:** Create individual test functions within the `tests` module. Each test function should be annotated with `#[test]`.

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::types::{Config, WorkerResult};

        #[test]
        fn test_my_new_plugin_success() {
            // Arrange: Set up test data and configurations
            let config = Config {
                args: "some_valid_input".to_string(),
                ..Default::default()
            };

            // Act: Call the function being tested
            let result = run(config);

            // Assert: Verify the results
            assert_eq!(result.message, "Expected success message");
            assert!(result.value > 0.0);
        }

        #[test]
        fn test_my_new_plugin_failure() {
            // Arrange
            let config = Config {
                args: "some_invalid_input".to_string(),
                ..Default::default()
            };

            // Act
            let result = run(config);

            // Assert
            assert_eq!(result.message, "Expected error message");
            assert_eq!(result.value, -1.0); // Or whatever indicates failure
        }
    }
    ```

5.  **Run the Tests:** To run all tests in the project, execute the following command in your terminal:

    ```bash
    cargo test
    ```

    To run tests for a specific module (e.g., `my_new_plugin`):

    ```bash
    cargo test my_new_plugin
    ```

## Best Practices

*   **Descriptive Test Names:** Name your test functions clearly to indicate what they are testing (e.g., `test_function_name_scenario`).
*   **Arrange, Act, Assert (AAA):** Structure your tests into these three phases for readability and maintainability.
*   **Isolate Tests:** Ensure each test is independent and does not rely on the state of other tests.
*   **Edge Cases:** Test not only successful scenarios but also edge cases, error conditions, and invalid inputs.
*   **Mocking (if necessary):** For complex plugins that interact with external systems (e.g., network requests, file system), consider using mocking libraries or techniques to isolate the unit under test.
