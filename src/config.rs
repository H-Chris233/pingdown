use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;

use crate::cli::Cli;
use crate::system::error;

/// Configuration parameter structure supporting JSON serialization/deserialization
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(alias = "address")]  // Alias mapping: JSON field -> struct field
    pub vec_address: Vec<String>,
    #[serde(default)]  // Allow missing field, use bool default (false)
    pub strict: bool,
    #[serde(alias = "secs-for-normal-loop", default = "default_60")]    // Alias | Default: 60
    pub secs_for_normal_loop: u64,
    #[serde(alias = "secs-for-emergency-loop", default = "default_20")]  // Alias | Default: 20
    pub secs_for_emergency_loop: u64,
    #[serde(alias = "times-for-emergency-loop", default = "default_3")] // Alias | Default: 3
    pub times_for_emergency_loop: u64,
}

/// Serde default functions (standalone for function pointer requirements, easier maintenance)
fn default_60() -> u64 {60}  // Normal loop interval
fn default_20() -> u64 {20}  // Emergency loop interval
fn default_3() -> u64 {3}    // Emergency loop count

/// Loads configuration from JSON file
/// 
/// # Path
/// - Hardcoded path: "./config.json"
/// 
/// # Error Handling
/// - File read failure: Terminates process via error()
/// - JSON parse failure: Terminates process via error()
pub fn read_json() -> Config {
    let json_str = match fs::read_to_string("./config.json") {
        Ok(json_str) => json_str,
        Err(err) => error(&format!("reading JSON file.\n{}", err)),
    };
    let config: Config = match serde_json::from_str(&json_str) {
        Ok(json_info) => json_info,
        Err(err) => error(&format!("parsing JSON file.\n{}", err)),
    };
    config
}

/// CLI arguments to config struct converter
/// 
/// # Conversion Logic
/// - Numeric fields: Safely converted via convert_num
/// - Address list: Direct mapping
/// - Strict mode: Direct mapping
pub fn from_cli(cli: Cli) -> Config {
    Config {
        secs_for_normal_loop: convert_num(&cli.secs_for_normal_loop),
        secs_for_emergency_loop: convert_num(&cli.secs_for_emergency_loop),
        times_for_emergency_loop: convert_num(&cli.times_for_emergency_loop),
        vec_address: cli.vec_address,
        strict: cli.strict,
    }
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
        Err(_) => error(&format!("Invalid numeric input[in function convert_num]\nExpected positive integer, found: {}", num)),
    };
    if num == 0 {
        error(&format!("Zero value detected[in function convert_num]\nPositive integer required, found: {}", num));
    }
    num
}

/// Configuration debugging interface
/// 
/// # Default Implementation
/// - Prints pretty-printed Debug output
/// - Displays initialization status
pub trait OutputInfo: Debug {
    /// Outputs structured debug info and initialization status
    fn output_info(&self) {
        println!("{:#?}", self);
        println!("Initializing monitoring process...");
    }
}

impl OutputInfo for Config {}
