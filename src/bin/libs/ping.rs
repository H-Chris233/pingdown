use crate::libs::io::{run_command, error};
use pingdown::Output;

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
pub fn check_status(vec_address: &Vec<String>, strict: &bool, output: &mut Output) -> bool {
    let mut status_vec: Vec<bool> = vec![];
    for ip in vec_address {
        let status = get_status(ip);
        status_vec.push(status);
    }
    let mut success: u64 = 0;
    let mut failure: u64 = 0;
    let status = match strict {
        false => {
            for status in status_vec {
                match status { // Default mode: any successful connection passes
                    true => success += 1,
                    false => failure += 1,
                }
            }
            if success > 0 {true} else{false}
        },
        true => {
            for status in status_vec {
                match status { // Strict mode: requires all connections to succeed
                    true => success += 1,
                    false => failure += 1,
                }
            }
            if failure > 0 {false} else{true}
        },
    };
    println!("Succeeds:{},\nFailures:{}", success, failure);
    status
}







