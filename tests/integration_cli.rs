use assert_cmd::cargo::CommandCargoExt;
use std::io::Write;
use std::process::{Command, Stdio};
use std::{thread, time::Duration};
use tempfile::TempDir;

#[test]
fn cli_uses_config_file_values() {
    let tmp = TempDir::new().unwrap();
    let cfg_path = tmp.path().join("config.json");
    std::fs::write(&cfg_path, r#"{
        "address": ["10.0.0.1"],
        "strict": false,
        "secs-for-normal-loop": 1,
        "secs-for-emergency-loop": 1,
        "times-for-emergency-loop": 1
    }"#).unwrap();

    let mut cmd: Command = Command::cargo_bin("pingdown").unwrap();
    cmd.arg("--config").arg(&cfg_path)
       .stdout(Stdio::piped())
       .stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("spawn pingdown");

    // Give it a short time to print configuration
    thread::sleep(Duration::from_millis(300));
    // Kill the process to avoid hanging test
    let _ = child.kill();
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Validate that the effective configuration printed the target from file
    assert!(stdout.contains("[CONFIG]"));
    assert!(stdout.contains("targets     : 10.0.0.1"));
}

#[test]
fn cli_parses_direct_args() {
    let mut cmd: Command = Command::cargo_bin("pingdown").unwrap();
    cmd.args(["-s", "-n", "1", "-e", "1", "-t", "1", "1.1.1.1"]) // strict with one target
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("spawn pingdown");
    thread::sleep(Duration::from_millis(300));
    let _ = child.kill();
    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[CONFIG]"));
    assert!(stdout.contains("targets     : 1.1.1.1"));
}

#[test]
fn metrics_persist_to_temp_dir() {
    use pingdown::runtime::Metrics;

    let tmp = TempDir::new().unwrap();
    let orig = std::env::current_dir().unwrap();
    std::env::set_current_dir(tmp.path()).unwrap();

    let metrics = Metrics { total_succeeds: 2, total_failures: 1, total_normal_loop_times: 3, total_emergency_loop_times: 0 };
    metrics.write();

    // Restore cwd to avoid affecting other tests
    std::env::set_current_dir(orig).unwrap();

    let contents = std::fs::read_to_string(tmp.path().join("pingdown_runtime_info.txt")).expect("runtime file present");
    assert!(contents.contains("total_succeeds: 2"));
}
