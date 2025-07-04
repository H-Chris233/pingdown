
use crate::libs::io::{run_command};
use crate::libs::output_file::{RuntimeInfo, Info, add_one};
use std::sync::{Arc, Mutex};
use anyhow::{Context, Result};
use log::{debug, error, info};

/// Tests connectivity to a single target using system ping command
fn get_status(ip: &str) -> Result<bool> {
    #[cfg(unix)]
    let command = format!("ping -c 1 {}", ip);

    #[cfg(windows)]
    let command = format!("ping -n 1 {}", ip);

    let message = format!("Pinging {}...", ip);
    let output = run_command(&command, Some(&message))
        .with_context(|| format!("Failed to execute ping command for {}", ip))?;
    
    let status = String::from_utf8_lossy(&output.stdout).to_string();
    let success = status.contains("TTL") || status.contains("ttl"); // Detect TTL for success
    
    if success {
        info!("Ping to {} succeeded", ip);
    } else {
        info!("Ping to {} timed out", ip);
    }
    
    Ok(success)
}

/// Evaluates connectivity status across multiple targets according to monitoring mode
pub fn check_status(vec_address: &Vec<String>, strict: &bool, runtime_info: &Arc<Mutex<RuntimeInfo>>) -> Result<bool> {
    let mut status_vec: Vec<(String, bool)> = vec![];

    for ip in vec_address {
        let status = match get_status(ip) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to get status for {}: {}", ip, e);
                false
            }
        };
        status_vec.push((ip.clone(), status));
    }

    let mut succeeds = 0;
    let mut failures = 0;

    for (ip, status) in &status_vec {
        match *status {
            true => {
                succeeds += 1;
                add_one(runtime_info, Info::Succeeds)
                    .with_context(|| format!("Failed to increment succeed counter for IP: {}", ip))?;
            }
            false => {
                failures += 1;
                add_one(runtime_info, Info::Failures)
                    .with_context(|| format!("Failed to increment failure counter for IP: {}", ip))?;
            }
        }
    }

    let result = match strict {
        true => {
            info!("Strict mode: All connections must succeed");
            failures == 0
        }
        false => {
            info!("Default mode: Any successful connection passes");
            succeeds > 0
        }
    };

    info!("Succeeds: {}\nFailures: {}", succeeds, failures);

    Ok(result)
}
