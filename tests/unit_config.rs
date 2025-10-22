use clap::Parser;
use pingdown::cli::Cli;
use pingdown::config::{build_monitor_config, ConfigError, ENV_CONFIG_PATH};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn cli_only_arguments_build_monitor_config() {
    let args = vec![
        "pingdown",
        "1.1.1.1",
        "-s",
        "-n",
        "30",
        "-e",
        "10",
        "-t",
        "5",
        "--status-only",
        "-q",
        "--progress",
        "-v",
        "-v",
    ];
    let cli = Cli::parse_from(args);

    let cfg = build_monitor_config(&cli).expect("cli configuration should succeed");
    assert_eq!(cfg.targets, vec!["1.1.1.1".to_string()]);
    assert!(cfg.strict);
    assert_eq!(cfg.normal_interval_secs(), 30);
    assert_eq!(cfg.emergency_interval_secs(), 10);
    assert_eq!(cfg.emergency_retry_attempts(), 5);
    assert!(cfg.status_only);
    assert!(cfg.quiet);
    assert!(cfg.progress);
    assert_eq!(cfg.verbose, 2);
}

#[test]
fn cli_values_override_file_settings() {
    let mut file = NamedTempFile::new().expect("temp file");
    write!(
        file,
        r#"{
            "address": ["file.example"],
            "strict": false,
            "secs-for-normal-loop": 60,
            "secs-for-emergency-loop": 8,
            "times-for-emergency-loop": 2,
            "status_only": true,
            "progress": true,
            "verbose": 1
        }"#
    )
    .unwrap();
    file.flush().unwrap();

    let args = vec![
        "pingdown",
        "--config",
        file.path().to_str().unwrap(),
        "cli.example",
        "-s",
        "-n",
        "15",
        "-e",
        "5",
        "-v",
        "-v",
    ];
    let cli = Cli::parse_from(args);

    let cfg = build_monitor_config(&cli).expect("combined configuration should succeed");
    assert_eq!(cfg.targets, vec!["cli.example".to_string()]); // CLI takes precedence for targets
    assert!(cfg.strict);
    assert_eq!(cfg.normal_interval_secs(), 15); // CLI overrides file
    assert_eq!(cfg.emergency_interval_secs(), 5); // CLI overrides file
    assert_eq!(cfg.emergency_retry_attempts(), 2); // File value retained
    assert!(cfg.status_only); // File value retained
    assert!(cfg.progress); // File value retained
    assert_eq!(cfg.verbose, 2); // CLI overrides file
}

#[test]
fn environment_variable_selects_configuration_file() {
    let mut file = NamedTempFile::new().expect("temp file");
    write!(
        file,
        r#"{
            "address": ["env.example"],
            "strict": true,
            "secs-for-normal-loop": 42,
            "secs-for-emergency-loop": 7,
            "times-for-emergency-loop": 4
        }"#
    )
    .unwrap();
    file.flush().unwrap();

    let previous = std::env::var(ENV_CONFIG_PATH).ok();
    std::env::set_var(ENV_CONFIG_PATH, file.path());

    let cli = Cli::parse_from(vec!["pingdown"]);
    let cfg = build_monitor_config(&cli).expect("env configuration should succeed");

    assert_eq!(cfg.targets, vec!["env.example".to_string()]);
    assert!(cfg.strict);
    assert_eq!(cfg.normal_interval_secs(), 42);
    assert_eq!(cfg.emergency_interval_secs(), 7);
    assert_eq!(cfg.emergency_retry_attempts(), 4);

    if let Some(value) = previous {
        std::env::set_var(ENV_CONFIG_PATH, value);
    } else {
        std::env::remove_var(ENV_CONFIG_PATH);
    }
}

#[test]
fn invalid_address_reports_precise_field_path() {
    let cli = Cli::parse_from(vec!["pingdown", "!!!bad!!!"]);
    let err = build_monitor_config(&cli).expect_err("invalid address should fail");

    match err {
        ConfigError::Validation { field_path, message } => {
            assert_eq!(field_path, "cli.targets[0]");
            assert!(message.contains("!!!bad!!!"));
        }
        other => panic!("unexpected error: {}", other),
    }
}

#[test]
fn zero_values_are_rejected_with_field_context() {
    // Zero normal interval
    let cli = Cli::parse_from(vec!["pingdown", "1.1.1.1", "-n", "0"]);
    let err = build_monitor_config(&cli).expect_err("zero normal interval should fail");
    match err {
        ConfigError::Validation { field_path, .. } => {
            assert_eq!(field_path, "cli --normal/-n");
        }
        other => panic!("unexpected error: {}", other),
    }

    // Zero retries
    let cli = Cli::parse_from(vec!["pingdown", "1.1.1.1", "-t", "0"]);
    let err = build_monitor_config(&cli).expect_err("zero retries should fail");
    match err {
        ConfigError::Validation { field_path, .. } => {
            assert_eq!(field_path, "cli --tries/-t");
        }
        other => panic!("unexpected error: {}", other),
    }
}

#[test]
fn file_validation_errors_preserve_field_paths() {
    let mut file = NamedTempFile::new().expect("temp file");
    write!(
        file,
        r#"{
            "address": ["file.target"],
            "secs-for-normal-loop": 10,
            "secs-for-emergency-loop": 5,
            "times-for-emergency-loop": 0
        }"#
    )
    .unwrap();
    file.flush().unwrap();

    let args = vec!["pingdown", "--config", file.path().to_str().unwrap()];
    let cli = Cli::parse_from(args);
    let err = build_monitor_config(&cli).expect_err("zero retries in file should fail");

    match err {
        ConfigError::Validation { field_path, .. } => {
            assert_eq!(field_path, format!("{}:times-for-emergency-loop", file.path().display()));
        }
        other => panic!("unexpected error: {}", other),
    }
}
