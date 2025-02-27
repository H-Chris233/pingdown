#![allow(dead_code)]
#![allow(unused)]

use pingdown::*;
use regex::Regex;

/// Handles argument processing. Adjusts terminal encoding on Windows. Acts as the program entry point.
fn main() {
    let cli = Cli::parse();
    #[cfg(windows)]
    cmd_to_utf8();
    
    check_cli(&cli);
    output_message(&cli);
    normal_loop(&cli.vec_ip, &cli);
}

///Use regex to verify if the address is correct or not.
fn check_cli(cli: &Cli) {
    let re_address = match Regex::new(r"^(?:(?:https?|ftp|file|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$") {
        Ok(re_ip) => re_ip,
        Err(err) => error(&format!("Regex compilation failed. {}[in function check_cli]", err)),
    };
    if cli.vec_ip.is_empty() {
        println!("Please input at least one ip or website.\nYou can also use -h or --help to get help.");
        sleep(3);
        std::process::exit(0);
    }
    for ip in &cli.vec_ip {
        match re_address.is_match(ip) {
            true => {},
            false => error(&format!("reading input address[in function check_cli]\n{}\nis this the correct address?", ip)),
        }
    }
    check_num(&cli.secs_for_normal_loop);
    check_num(&cli.secs_for_emergency_loop);
    check_num(&cli.times_for_emergency_loop);
}

///Check if the value is a nature number or not. 
fn check_num(num: &str) {
    let num: u64 = match num.parse() {
        Ok(num) => num,
        Err(_) => error(&format!("checking input number.\nDon't be crazy,{} is not a number.\n[in function check_num]", num)),
    };
    if num == 0 {
        error(&format!("checking input number.\nDon't be crazy,{} is not a lucky number.\n[in function check_num]", num));
    }
}









