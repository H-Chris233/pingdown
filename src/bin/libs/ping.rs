use crate::libs::io::{run_command, error};
use crate::libs::output_file::{RuntimeInfo, Info, add_one};
use std::sync::{Arc, Mutex};

/// Tests connectivity to a single target using system ping command
fn get_status(ip: &str) -> bool {
    #[cfg(unix)]
    let command = format!("ping -c 1 {}", ip);

    #[cfg(windows)]
    let command = format!("ping -n 1 {}", ip);

    let message = format!("Pinging {}...", ip);
    let output = match run_command(&command, Some(&message)) {
        Ok(output) => output, // Gets command output
        Err(_) => error("executing command[in get_status]"),
    };
    let status = String::from_utf8_lossy(&output.stdout).to_string(); // Converts byte stream to UTF-8 string with invalid character handling
    if status.contains("TTL") || status.contains("ttl") { // Checks for TTL presence to determine success
         println!("Success.");
         true
    } else {
         println!("Request timed out.");
         false // indicates connection failure state
    }
}

/// Evaluates connectivity status across multiple targets according to monitoring mode
pub fn check_status(vec_address: &Vec<String>, strict: &bool, runtime_info: &Arc<Mutex<RuntimeInfo>>) -> bool {
    let mut status_vec: Vec<bool> = vec![];
    for ip in vec_address {
        let status = get_status(ip);
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
                        add_one(runtime_info, Info::Succeeds);
                    }
                    false => {
                        failures += 1;
                        add_one(runtime_info, Info::Failures);
                    }
                }
            }
            if succeeds > 0 {true} else{false}// Default mode: any successful connection passes
        },
        true => {
            for status in status_vec {
                match status { 
                    true => {
                        succeeds += 1;
                        add_one(runtime_info, Info::Succeeds);
                    }
                    false => {
                        failures += 1;
                        add_one(runtime_info, Info::Failures);
                    }
                }
            }
            if failures > 0 {false} else{true}// Strict mode: requires all connections to succeed
        },
    };
    println!("Succeeds:{},\nFailures:{}", succeeds, failures);
    status
}












