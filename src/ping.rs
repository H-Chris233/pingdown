use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::runtime::{add_one, MetricEvent, Metrics};
use crate::system::{error, System};

/// Tests connectivity to a single target using system ping command
fn get_status<S: System>(ip: &str, system: &S, cfg: &Config) -> bool {
    let command = system.build_ping_command(ip);
    let message = if cfg.quiet || cfg.status_only { None } else { Some(format!("Pinging {}...", ip)) };
    let output = match system.run_shell_command(&command, message.as_deref()) {
        Ok(output) => output,
        Err(_) => error("executing command[in get_status]"),
    };
    let status = String::from_utf8_lossy(&output.stdout).to_string();
    let ok = status.contains("TTL") || status.contains("ttl");
    if cfg.verbose > 0 && !cfg.status_only && !cfg.quiet {
        if ok { println!("Success."); } else { println!("Request timed out."); }
    }
    ok
}

/// Evaluates connectivity status across multiple targets according to monitoring mode
pub fn check_status<S: System>(cfg: &Config, metrics: &Arc<Mutex<Metrics>>, system: &S) -> (bool, u64, u64) {
    let mut status_vec: Vec<bool> = vec![];
    for ip in &cfg.vec_address {
        let status = get_status(ip, system, cfg);
        status_vec.push(status);
    }
    let mut succeeds = 0;
    let mut failures = 0;
    let status = if cfg.strict {
        for status in status_vec {
            match status {
                true => { succeeds += 1; add_one(metrics, MetricEvent::Succeeds); }
                false => { failures += 1; add_one(metrics, MetricEvent::Failures); }
            }
        }
        failures <= 0
    } else {
        for status in status_vec {
            match status {
                true => { succeeds += 1; add_one(metrics, MetricEvent::Succeeds); }
                false => { failures += 1; add_one(metrics, MetricEvent::Failures); }
            }
        }
        succeeds > 0
    };
    (status, succeeds, failures)
}
