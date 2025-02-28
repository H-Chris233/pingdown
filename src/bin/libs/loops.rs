use pingdown::Cli;
use crate::libs::ping::check_status;
use crate::libs::io::{sleep, error, shutdown};

/// Continuously monitors connectivity in regular intervals
pub fn normal_loop(vec_ip: &Vec<String>, cli: &Cli) {
    let secs: u64 = match cli.secs_for_normal_loop.parse() {
        Ok(secs) => secs, // Parses user-defined interval duration
        Err(_) => {
            println!("Please check your input.");
            error("parsing interval duration [normal_loop context]")
        }
    };
    println!("Started {}sec loop...", secs);
    for i in 1.. {
        let status = check_status(vec_ip, &cli.strict);
        if status == false {
            emergency_loop(vec_ip, cli);
            continue;
        }
        println!("Normal looped for {} times...", i);
        println!("{} secs left for the next normal loop...", secs);
        sleep(secs);
    }
}

/// Critical failure handler activated when connectivity is lost. Implements retry mechanism and system shutdown protocol.
fn emergency_loop(vec_ip: &Vec<String>, cli: &Cli) {
    let secs: u64 = match cli.secs_for_emergency_loop.parse() {
        Ok(secs) => secs, // Parses emergency retry interval duration
        Err(_) => {
            println!("Please check your input.");
            error("parsing emergency interval [emergency_loop]");
        }
    };
    let mut time_left: usize = match cli.times_for_emergency_loop.parse() {
        Ok(time_left) => time_left,
        Err(_) => {
            println!("Please check your input.");
            error("converting input to number[in emergency_loop]");
        }
    };
    println!("Warning!!! Connection lost!!!!");
    println!("Checking connection every {} seconds!!", secs);
    loop {
        println!("{} tries remaining...", time_left);
        let status = check_status(vec_ip, &cli.strict);
        if status == true {
            break;
        } else if time_left <= 0 {
            shutdown();
            error("system shutdown failed - check permissions"); // System still running indicates permission issues
        }

        println!("{} secs left for the next check...", secs);
        sleep(secs);
        time_left -= 1;
    }
    println!("Reconnected!!!");
    println!("Exiting {}sec emergency loop...", secs);
}
