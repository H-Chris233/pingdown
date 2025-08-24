
use std::process::Command;
use std::time::Duration;
use std::thread;
use std::process::Output;
use anyhow::{anyhow, Context, Result};
use log::{error, info};

/// Unix command line execution (sh)
#[cfg(unix)]
pub fn run_command(command: &str, message: Option<&str>) -> Result<Output> {
    if let Some(message) = message {
        info!("{}", message);
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .context(format!("Failed to execute command: {}", command))?;
    Ok(output)
}

/// Windows command line execution (cmd)
#[cfg(windows)]
pub fn run_command(command: &str, message: Option<&str>) -> Result<Output> {
    if let Some(message) = message {
        info!("{}", message);
    }
    let output = Command::new("cmd")
        .arg("/C")
        .arg(command)
        .output()
        .context(format!("Failed to execute command: {}", command))?;
    Ok(output)
}

/// Unix shutdown command implementation with fallback methods
#[cfg(unix)]
pub fn shutdown() -> Result<()> {
    let _ = system_shutdown::shutdown();
    sleep(7);

    let commands = [
        ("shutdown -h now", Some("Starting shutdown...")),
        ("poweroff", None),
        ("poweroff -f", None),
        ("halt", None),
        ("init 0", None),
        ("systemctl poweroff", None),
    ];

    for &(cmd, msg) in &commands {
        run_command(cmd, msg)?;
        sleep(7);
    }

    Err(anyhow!("All shutdown commands failed"))
}

/// Windows shutdown command implementation
#[cfg(windows)]
pub fn shutdown() -> Result<()> {
    let _ = system_shutdown::shutdown();
    sleep(7);

    run_command("shutdown /s /t 0", Some("Starting shutdown..."))
        .map_err(|e| {
            error!("Windows shutdown failed: {}", e);
            e
        })
        .context("Windows shutdown failed")
}

/// Configures Windows console for UTF-8 text encoding
#[cfg(windows)]
pub fn cmd_to_utf8() {
    if let Err(e) = run_command("chcp 65001", None) {
        error!("Failed to configure UTF-8 encoding: {}", e);
    }
}

/// Suspends execution for specified duration
pub fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}
