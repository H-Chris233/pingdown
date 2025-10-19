use std::sync::{Arc, Mutex};

use crate::runtime::{add_one, MetricEvent, Metrics};
use crate::system::{error, System};

/// Tests connectivity to a single target using system ping command
fn get_status<S: System>(ip: &str, system: &S) -> bool {
    let command = system.build_ping_command(ip);
    let message = format!("Pinging {}...", ip);
    let output = match system.run_shell_command(&command, Some(&message)) {
        Ok(output) => output,
        Err(_) => error("executing command[in get_status]"),
    };
    let status = String::from_utf8_lossy(&output.stdout).to_string();
    if status.contains("TTL") || status.contains("ttl") {
         println!("Success.");
         true
    } else {
         println!("Request timed out.");
         false
    }
}

/// Evaluates connectivity status across multiple targets according to monitoring mode
pub fn check_status<S: System>(vec_address: &Vec<String>, strict: &bool, metrics: &Arc<Mutex<Metrics>>, system: &S) -> bool {
    let mut status_vec: Vec<bool> = vec![];
    for ip in vec_address {
        let status = get_status(ip, system);
        status_vec.push(status);
    }
    let mut succeeds = 0;
    let mut failures = 0;
    let status = match strict {
        false => {
            for status in status_vec {
                match status { 
                    true => {
                        succeeds += 1;
                        add_one(metrics, MetricEvent::Succeeds);
                    }
                    false => {
                        failures += 1;
                        add_one(metrics, MetricEvent::Failures);
                    }
                }
            }
            succeeds > 0
        },
        true => {
            for status in status_vec {
                match status { 
                    true => {
                        succeeds += 1;
                        add_one(metrics, MetricEvent::Succeeds);
                    }
                    false => {
                        failures += 1;
                        add_one(metrics, MetricEvent::Failures);
                    }
                }
            }
            failures <= 0
        },
    };
    println!("Succeeds:{},\nFailures:{}", succeeds, failures);
    status
}
