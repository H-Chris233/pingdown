use crate::libs::io::{run_command, error};

/// Tests connectivity to a single target using system ping command
fn get_status(ip: &str) -> bool {
    #[cfg(unix)]
    let command = format!("ping -c 1 {}", ip);

    #[cfg(windows)]
    let command = format!("ping -n 1 {}", ip);

    let message = format!("Started pinging {}...", ip);
    let output = match run_command(&command, Some(&message)) {
        Ok(output) => output, // Gets command output
        Err(_) => error("executing command[in get_status]"),
    };
    let status = String::from_utf8_lossy(&output.stdout).to_string(); // Converts byte stream to UTF-8 string with invalid character handling
    println!("Started checking {}...", ip);
    if status.contains("TTL") || status.contains("ttl") { // Checks for TTL presence to determine success
         println!("Success.");
         true
    } else {
         println!("Request timed out.");
         false // indicates connection failure state
    }
}

/// Evaluates connectivity status across multiple targets according to monitoring mode
pub fn check_status(vec_address: &Vec<String>, strict: &bool) -> bool {
    let mut status_vec: Vec<bool> = vec![];
    for ip in vec_address {
        let status = get_status(ip);
        status_vec.push(status);
    }
    let status = match strict {
        false => {
            match status_vec.contains(&true) { // Default mode: any successful connection passes
                true => true,
                false => false,
            }
        },
        true => {
            match status_vec.contains(&false) { // Strict mode: requires all connections to succeed
                true => false,
                false => true,
            }
        },
    };
    status
}
