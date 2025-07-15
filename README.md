# jr

`jr` is a command-line tool for running periodic tests and sending the results to various outputs. It is designed to be extensible through a simple plugin system.

## Configuration

`jr` is configured through the `jr.conf` file. Each line in this file defines a test to be run, with the following format:

```
name::seconds::test::arguments
```

- `name`: A unique name for the test.
- `seconds`: The interval in seconds at which to run the test.
- `test`: The name of the worker plugin to execute.
- `arguments`: A string of arguments to pass to the worker plugin.

**Example `jr.conf`:**

```
load_avg::30::load_avg::mordor
check_session_endpoint::300::check_url::https://www.clinique.com//rest/api/v1/user/session?brand=2&region=0
saltogrande_gefs_00::6::timethis::sleep 3
```

## Usage

To run `jr`, simply execute the binary:

```bash
./jr
```

`jr` also accepts command-line arguments to override or supplement the configuration file.

- `--every <SECONDS>`: Overrides the interval for all tests.
- `--once`: Runs all tests once and then exits.
- `--name <NAME>`: Runs only the test with the specified name.

## Plugins

`jr` has two types of plugins:

- **Worker Plugins:** These perform the actual tests, such as checking a URL, measuring CPU load, or timing a command.
- **Output Plugins:** These take the results from the worker plugins and send them to different destinations, such as standard output, Graphite, or a web service.
