use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use colored::Colorize;

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

    // UX/Output flags (also configurable via JSON for convenience)
    #[serde(default)]
    pub quiet: bool,
    #[serde(default)]
    pub status_only: bool,
    #[serde(default)]
    pub progress: bool,
    #[serde(default = "default_verbose")] // clap -v count style (0,1,2,...)
    pub verbose: u8,
}

/// Serde default functions (standalone for function pointer requirements, easier maintenance)
fn default_60() -> u64 {60}  // Normal loop interval
fn default_20() -> u64 {20}  // Emergency loop interval
fn default_3() -> u64 {3}    // Emergency loop count
fn default_verbose() -> u8 {0}

/// Loads configuration from JSON file
///
/// # Path
/// - Path may be provided; defaults to "./config.json"
///
/// # Error Handling
/// - File read failure: Terminates process via error()
/// - JSON parse failure: Terminates process via error()
pub fn read_json_with_path<P: AsRef<Path>>(path: Option<P>) -> Config {
    let path_buf;
    let path_ref: &Path = if let Some(p) = path {
        path_buf = p.as_ref().to_path_buf();
        &path_buf
    } else {
        Path::new("./config.json")
    };

    let json_str = match fs::read_to_string(path_ref) {
        Ok(json_str) => json_str,
        Err(err) => error(&format!("reading JSON file.\n{}", err)),
    };
    let config: Config = match serde_json::from_str(&json_str) {
        Ok(json_info) => json_info,
        Err(err) => error(&format!("parsing JSON file.\n{}", err)),
    };
    config
}

/// Backwards compatible function for legacy callers
pub fn read_json() -> Config { read_json_with_path::<&Path>(None) }

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
        quiet: cli.quiet,
        status_only: cli.status_only,
        progress: cli.progress,
        verbose: cli.verbose,
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

impl OutputInfo for Config {
    fn output_info(&self) {
        println!("{} Effective configuration", "[CONFIG]".bold());
        println!("  targets     : {}", self.vec_address.join(", "));
        println!("  strict      : {}", self.strict);
        println!("  normal      : {}s", self.secs_for_normal_loop);
        println!("  emergency   : {}s x{}", self.secs_for_emergency_loop, self.times_for_emergency_loop);
        println!("  verbose     : {}", self.verbose);
        println!("  quiet       : {}", self.quiet);
        println!("  status-only : {}", self.status_only);
        println!("  progress    : {}", self.progress);
        println!("{} Initializing monitoring process...", "[INIT]".bold());
    }
}
