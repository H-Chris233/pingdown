//! Configuration processing module: Handles config file reading, parameter conversion, struct transformations
use pingdown::{Info, Cli, RuntimeInfo};
use std::fs;
use crate::libs::io::error;
use std::fmt::Debug;

/// Loads configuration from JSON file
/// 
/// # Path
/// - Hardcoded path: "./config.json"
/// 
/// # Error Handling
/// - File read failure: Terminates process via error()
/// - JSON parse failure: Terminates process via error()
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

/// Numeric parameter validator/converter
/// 
/// # Functionality
/// 1. Converts string to u64 integer
/// 2. Validates non-zero positive integer
/// 
/// # Error Handling
/// - Parse failure: Triggers error() with invalid input
/// - Zero value: Triggers error() requiring positive integer
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

/// CLI arguments to config struct converter
/// 
/// # Conversion Logic
/// - Numeric fields: Safely converted via convert_num
/// - Address list: Direct mapping
/// - Strict mode: Direct mapping
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

/// Configuration debugging interface
/// 
/// # Default Implementation
/// - Prints pretty-printed Debug output
/// - Displays initialization status
pub trait StructInfo: Debug {
    /// Outputs structured debug info and initialization status
    fn output_info(&self) {
        println!("{:#?}", self);
        println!("Initializing monitoring process...");
    }
}

// Implements debug interface for CLI and config structs
impl StructInfo for Cli {}
impl StructInfo for Info {}









