use regex::Regex;
use crate::libs::io::error;
use pingdown::Cli;
use crate::libs::struct_info::convert_num;

/// Validates command-line inputs using regular expressions
/// Ensures proper IP/URL formatting and numeric parameter validity
pub fn check_cli(cli: &Cli) {
    let re_address = match Regex::new(r"^(?:(?:https?|ftp|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$") {
        Ok(re_ip) => re_ip,
        Err(err) => error(&format!("Regex compilation failed. {}[in function check_cli]", err)),
    };
    if cli.vec_address.is_empty() {
        println!("Please provide at least one IP address or website.\nFor usage instructions, use -h or --help.");
        error("there's no address to detect");
    }
    for ip in &cli.vec_address {
        match re_address.is_match(ip) {
            true => {},
            false => error(&format!("Invalid address format[in function check_cli]\n{}: Please verify target correctness", ip)),
        }
    }
    check_num(&cli.secs_for_normal_loop);
    check_num(&cli.secs_for_emergency_loop);
    check_num(&cli.times_for_emergency_loop);
}

/// Check if the number is correct natural number.
fn check_num(num: &str) {
    let _: u64 = match num.parse() {
        Ok(num) => num,
        Err(_) => error(&format!("Invalid numeric input[in function check_num]\nExpected positive integer, found: {}", num)),
    };
    if convert_num(num) == 0 {
        error(&format!("Zero value detected[in function check_num]\nPositive integer required, found: {}", num));
    }
}















