//! Core entry point and monitoring logic for pingdown network diagnostic tool
//! 
//! Key responsibilities:
//! - Signal handler initialization
//! - CLI argument parsing
//! - Configuration generation
//! - Main monitoring loop orchestration
//!
//! Core data structures:
//! - [`Cli`] for command-line inputs
//! - [`RuntimeInfo`] tracking runtime state
//! - [`Info`] storing monitoring parameters

#![allow(unused_imports)]

mod libs;

use crate::libs::check_input::check_cli;
use crate::libs::loops::normal_loop;
use crate::libs::ctrlc::ctrlc_init;
use crate::libs::struct_info::*;
use pingdown::Cli;
use clap::Parser;
use anyhow::{Result, Context};
use log::{debug, error, info};

#[cfg(windows)]
use crate::libs::io::cmd_to_utf8;

/// Main execution entry for network monitoring
///
/// Operational workflow:
/// 1. Parse command-line arguments
/// 2. Initialize signal handlers
/// 3. Load configuration (JSON file or CLI conversion)
/// 4. Prepare terminal environment(Windows only)
/// 5. Launch monitoring loop
///
/// # Implementation Notes
/// - Windows console UTF-8 encoding fix
/// - JSON configuration takes priority over CLI args
/// - Graceful exit via signal handling
fn main() -> Result<()> {
    // Initialize logging system
    simple_logger::SimpleLogger::new().with_level(log::LevelFilter::Info).init()?;
    
    // Parse CLI arguments into structured format
    let cli = Cli::parse();
    info!("CLI arguments parsed successfully");
    
    // Initialize CTRL-C handler with runtime state tracker
    let runtime_info = ctrlc_init().context("Failed to initialize Ctrl+C handler")?;
    
    // Configuration loading strategy:
    // - Load from JSON when --read-json flag present
    // - Otherwise validate and convert CLI arguments
    let info = if cli.read_json {
        info!("Loading configuration from JSON file");
        read_json().context("Failed to load JSON configuration")?
    } else {
        info!("Validating CLI arguments");
        check_cli(&cli).context("CLI arguments validation failed")?;
        info!("Converting CLI arguments to config");
        cli_to_info(cli).context("Failed to convert CLI arguments to config")?
    };

    // Windows console UTF-8 encoding enforcement
    #[cfg(windows)]
    {
        info!("Configuring Windows console for UTF-8");
        if let Err(e) = cmd_to_utf8() {
            error!("Failed to set UTF-8 mode: {}", e);
        }
    }

    // Output validated configuration and start monitoring
    info.output_info();
    info!("Starting normal monitoring loop");
    normal_loop(info, runtime_info).context("Monitoring loop failed")
}
