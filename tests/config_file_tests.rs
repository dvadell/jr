use std::fs;
use std::io::Write;

use crate::config::file::{parse_config, Config};

#[test]
fn test_parse_config() {
    let config_content = r#"
Group group1
10::load_avg::localhost
20::check_url::https://example.com

# This is a comment
30::timethis::echo hello

// Another comment
40::load_avg::localhost

Group group2
50::check_url::https://another-example.com
60::timethis::echo world
"#;

    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("jr.conf");
    fs::write(&config_path, config_content).unwrap();

    let configs = parse_config();

    assert_eq!(configs.len(), 6);

    assert_eq!(configs[0].n, 10);
    assert_eq!(configs[0].function, "load_avg");
    assert_eq!(configs[0].args, "localhost");
    assert_eq!(configs[0].short_name, "10::load_avg::localhost");
    assert_eq!(configs[0].group, "group1");

    assert_eq!(configs[1].n, 20);
    assert_eq!(configs[1].function, "check_url");
    assert_eq!(configs[1].args, "https://example.com");
    assert_eq!(configs[1].short_name, "20::check_url::https://example.com");
    assert_eq!(configs[1].group, "group1");

    assert_eq!(configs[2].n, 30);
    assert_eq!(configs[2].function, "timethis");
    assert_eq!(configs[2].args, "echo hello");
    assert_eq!(configs[2].short_name, "30::timethis::echo hello");
    assert_eq!(configs[2].group, "group1");

    assert_eq!(configs[3].n, 40);
    assert_eq!(configs[3].function, "load_avg");
    assert_eq!(configs[3].args, "localhost");
    assert_eq!(configs[3].short_name, "40::load_avg::localhost");
    assert_eq!(configs[3].group, "group1");

    assert_eq!(configs[4].n, 50);
    assert_eq!(configs[4].function, "check_url");
    assert_eq!(configs[4].args, "https://another-example.com");
    assert_eq!(configs[4].short_name, "50::check_url::https://another-example.com");
    assert_eq!(configs[4].group, "group2");

    assert_eq!(configs[5].n, 60);
    assert_eq!(configs[5].function, "timethis");
    assert_eq!(configs[5].args, "echo world");
    assert_eq!(configs[5].short_name, "60::timethis::echo world");
    assert_eq!(configs[5].group, "group2");

    temp_dir.close().unwrap();
}
