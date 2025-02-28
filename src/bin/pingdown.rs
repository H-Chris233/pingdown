#![allow(dead_code)]
#![allow(unused)]

mod libs;

use pingdown::*;
use regex::Regex;
use crate::libs::loops::normal_loop;
use crate::libs::io::*;

/// Handles command-line argument processing and terminal encoding configuration.
/// Serves as the main entry point for the application.
fn main() {
    let cli = Cli::parse();
    check_cli(&cli);
    output_message(&cli);
    #[cfg(windows)]
    cmd_to_utf8();
    
    normal_loop(&cli.vec_ip, &cli);
}

/// Displays configuration details and initialization status
fn output_message(cli: &Cli) {
    println!("{:#?}", cli);
    println!("Initializing monitoring process...");
}

/// Validates command-line inputs using regular expressions
/// Ensures proper IP/URL formatting and numeric parameter validity
fn check_cli(cli: &Cli) {
    let re_address = match Regex::new(r"^(?:(?:https?|ftp|file|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$") {
        Ok(re_ip) => re_ip,
        Err(err) => error(&format!("Regex compilation failed. {}[in function check_cli]", err)),
    };
    if cli.vec_ip.is_empty() {
        println!("Please provide at least one IP address or website.\nFor usage instructions, use -h or --help.");
        sleep(3);
        std::process::exit(0);
    }
    for ip in &cli.vec_ip {
        match re_address.is_match(ip) {
            true => {},
            false => error(&format!("Invalid address format[in function check_cli]\n{}: Please verify target correctness", ip)),
        }
    }
    check_num(&cli.secs_for_normal_loop);
    check_num(&cli.secs_for_emergency_loop);
    check_num(&cli.times_for_emergency_loop);
}

/// Verifies numeric parameters are valid positive integers
fn check_num(num: &str) {
    let num: u64 = match num.parse() {
        Ok(num) => num,
        Err(_) => error(&format!("Invalid numeric input[in function check_num]\nExpected positive integer, found: {}", num)),
    };
    if num == 0 {
        error(&format!("Zero value detected[in function check_num]\nPositive integer required, found: {}", num));
    }
}






