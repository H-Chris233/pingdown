use pingdown::{cli::Cli, config::{from_cli, read_json_with_path, Config}};
use clap::Parser;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn config_from_cli_parses_values() {
    let args = vec![
        "pingdown",
        "1.1.1.1",
        "8.8.8.8",
        "-s",
        "-n", "30",
        "-e", "10",
        "-t", "5",
        "--status-only",
        "-q",
    ];
    let cli = Cli::parse_from(args);
    let cfg = from_cli(cli);
    assert_eq!(cfg.vec_address, vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()]);
    assert!(cfg.strict);
    assert_eq!(cfg.secs_for_normal_loop, 30);
    assert_eq!(cfg.secs_for_emergency_loop, 10);
    assert_eq!(cfg.times_for_emergency_loop, 5);
    assert!(cfg.status_only);
    assert!(cfg.quiet);
}

#[test]
fn read_json_with_aliases_and_defaults() {
    let mut file = NamedTempFile::new().expect("temp file");
    let json = r#"{
        "address": ["127.0.0.1", "bing.com"],
        "strict": true,
        "secs-for-normal-loop": 12,
        "secs-for-emergency-loop": 3,
        "times-for-emergency-loop": 2
    }"#;
    write!(file, "{}", json).unwrap();

    let cfg: Config = read_json_with_path(Some(file.path()));
    assert_eq!(cfg.vec_address, vec!["127.0.0.1", "bing.com"]);
    assert!(cfg.strict);
    assert_eq!(cfg.secs_for_normal_loop, 12);
    assert_eq!(cfg.secs_for_emergency_loop, 3);
    assert_eq!(cfg.times_for_emergency_loop, 2);
    // Defaults for UX flags
    assert!(!cfg.quiet && !cfg.status_only && !cfg.progress);
    assert_eq!(cfg.verbose, 0);
}
