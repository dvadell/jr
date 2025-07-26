use std::fs;

use crate::types::Config;

fn parse_line(line: &str, curr_group: &str) -> Option<Config> {
    // Trim any leading/trailing whitespace and split the line by "::" up to 3 parts
    let mut parts = line.trim().splitn(4, "::");
    
    if line.trim().is_empty() || line.trim_start().starts_with('#') || line.trim_start().starts_with("//") {
        // Ignore empty lines and comment lines starting with # or //
        None
    } else if let (Some(short_name), Some(n_str), Some(function), Some(args)) = (parts.next(), parts.next(), parts.next(), parts.next()) {
        // Parse the first part as an unsigned 64-bit integer (`n`)
        if let Ok(n) = n_str.parse::<u64>() {
            if n == 0 {
                eprintln!("Invalid N value in config file at line: {}", line);
                return None;
            }
            Some(Config {
                n,
                function: function.to_string(),
                args: args.to_string(),
                short_name: short_name.to_string(),
                group: curr_group.to_string(),
                ..Default::default()
            })
        } else {
            eprintln!("Failed to parse N value in config file at line: {}", line);
            None
        }
    } else {
        eprintln!("Invalid format in config file at line: {}", line);
        None
    }
}

pub fn parse_config() -> Vec<Config> {
    // Initialize a vector to store Config structures
    let mut configs: Vec<Config> = Vec::new();
    let mut curr_group = "Default";
    
    // Read the configuration file. Return empty configs if no config file.
    let config = match fs::read_to_string("jr.conf") {
        Ok(content) => content,
        Err(_) => return configs
    };
    
    // Split the content into lines
    let lines: Vec<&str> = config.trim().lines().collect();
    
    // Iterate over each line and parse it into a Config structure
    for line in lines {
        if line.trim_start().starts_with("Group") {
            if let Some(group_name) = line.split_whitespace().nth(1) {
                curr_group = group_name;
            }
        } else {
            if let Some(config) = parse_line(line, curr_group) {
                configs.push(config);
            }
        }
    }
    configs
}

#[test]
fn test_parse_config() {
    let config_content = r#"
Group group1
test1::10::load_avg::localhost
test2::20::check_url::https://example.com

# This is a comment
test3::30::timethis::echo hello

// Another comment
test4::40::load_avg::localhost

Group group2
test5::50::check_url::https://another-example.com
test6::60::timethis::echo world
"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("jr.conf");
    fs::write(&config_path, config_content).unwrap();

    // Go to the temp directory and load the configuration.
    use std::env;
    let _ = env::set_current_dir(&temp_dir);
    let configs = parse_config();

    assert_eq!(configs.len(), 6);

    assert_eq!(configs[0].n, 10);
    assert_eq!(configs[0].function, "load_avg");
    assert_eq!(configs[0].args, "localhost");
    assert_eq!(configs[0].short_name, "test1");
    assert_eq!(configs[0].group, "group1");

    assert_eq!(configs[1].n, 20);
    assert_eq!(configs[1].function, "check_url");
    assert_eq!(configs[1].args, "https://example.com");
    assert_eq!(configs[1].short_name, "test2");
    assert_eq!(configs[1].group, "group1");

    assert_eq!(configs[2].n, 30);
    assert_eq!(configs[2].function, "timethis");
    assert_eq!(configs[2].args, "echo hello");
    assert_eq!(configs[2].short_name, "test3");
    assert_eq!(configs[2].group, "group1");

    assert_eq!(configs[3].n, 40);
    assert_eq!(configs[3].function, "load_avg");
    assert_eq!(configs[3].args, "localhost");
    assert_eq!(configs[3].short_name, "test4");
    assert_eq!(configs[3].group, "group1");

    assert_eq!(configs[4].n, 50);
    assert_eq!(configs[4].function, "check_url");
    assert_eq!(configs[4].args, "https://another-example.com");
    assert_eq!(configs[4].short_name, "test5");
    assert_eq!(configs[4].group, "group2");

    assert_eq!(configs[5].n, 60);
    assert_eq!(configs[5].function, "timethis");
    assert_eq!(configs[5].args, "echo world");
    assert_eq!(configs[5].short_name, "test6");
    assert_eq!(configs[5].group, "group2");

    temp_dir.close().unwrap();
}

