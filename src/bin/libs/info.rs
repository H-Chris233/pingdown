use pingdown::{Info, Cli};
use std::fs;
use crate::libs::io::error;

pub fn read_json() -> Info {
    let json_str = match fs::read_to_string("./config.json") {
        Ok(json_str) => json_str,
        Err(err) => error(&format!("reading JSON file.\n{}", err)),
    };
    let config: Info = match serde_json::from_str(&json_str) {
        Ok(json_info) => json_info,
        Err(err) => error(&format!("parsing JSON file.\n{}", err)),
    };
    config
}

/// Displays configuration details and initialization status
pub fn output_info(info: &Info) {
    println!("{:#?}", info);
    println!("Initializing monitoring process...");
}

/// Verifies numeric parameters are valid positive integers
pub fn convert_num(num: &str) -> u64 {
    let num: u64 = match num.parse() {
        Ok(num) => num,
        Err(_) => error(&format!("Invalid numeric input[in function check_num]\nExpected positive integer, found: {}", num)),
    };
    if num == 0 {
        error(&format!("Zero value detected[in function check_num]\nPositive integer required, found: {}", num));
    }
    num
}

/// Turn a Cli struct to a Info struct.
pub fn cli_to_info(cli: Cli) -> Info {
    let output = Info {
        secs_for_normal_loop: convert_num(&cli.secs_for_normal_loop),
        secs_for_emergency_loop: convert_num(&cli.secs_for_emergency_loop),
        times_for_emergency_loop: convert_num(&cli.times_for_emergency_loop),
        vec_address: cli.vec_address,
        strict: cli.strict,
    };
    output
}







