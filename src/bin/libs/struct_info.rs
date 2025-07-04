
use pingdown::{JsonInfo, Cli};
use std::fs;
use std::fmt::Debug;
use anyhow::{Context, Result};
use log::{debug, error, info};

/// Loads configuration from JSON file
/// 
/// # Path
/// - Hardcoded path: "./config.json"
pub fn read_json() -> Result<JsonInfo> {
    let json_str = fs::read_to_string("./config.json")
        .with_context(|| format!("Failed to read configuration file: ./config.json"))?;
    
    serde_json::from_str(&json_str)
        .with_context(|| format!("Failed to parse JSON configuration"))
}

/// CLI arguments to config struct converter
pub fn cli_to_info(cli: Cli) -> Result<JsonInfo> {
    Ok(JsonInfo {
        secs_for_normal_loop: convert_num(&cli.secs_for_normal_loop)?,
        secs_for_emergency_loop: convert_num(&cli.secs_for_emergency_loop)?,
        times_for_emergency_loop: convert_num(&cli.times_for_emergency_loop)?,
        vec_address: cli.vec_address,
        strict: cli.strict,
    })
}

/// Configuration debugging interface with structured logging
pub trait StructInfo: Debug {
    /// Outputs structured debug info and initialization status
    fn output_info(&self) {
        info!("Configuration: {:#?}", self);
        info!("Initializing monitoring process...");
    }
}

// Implements debug interface for CLI and config structs
impl StructInfo for Cli {}
impl StructInfo for JsonInfo {}

/// Numeric parameter validator/converter
/// 
/// # Functionality
/// 1. Converts string to u64 integer
/// 2. Validates non-zero positive integer
pub fn convert_num(num: &str) -> Result<u64> {
    let parsed: u64 = num.parse()
        .map_err(|_| anyhow::anyhow!("Invalid numeric input: expected positive integer, found '{}'", num))?;
    
    if parsed == 0 {
        Err(anyhow::anyhow!("Zero value detected: positive integer required"))
    } else {
        debug!("Successfully converted number: {}", parsed);
        Ok(parsed)
    }
}
