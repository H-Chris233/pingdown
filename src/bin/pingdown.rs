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

mod libs;

use crate::libs::check_input::check_cli;
use crate::libs::loops::normal_loop;
use crate::libs::ctrlc::ctrlc_init;
use crate::libs::struct_info::*;
use pingdown::Cli;
use clap::Parser;

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
fn main() {
    // Parse CLI arguments into structured format
    let cli = Cli::parse();
    
    // Initialize CTRL-C handler with runtime state tracker
    let runtime_info = ctrlc_init();
    
    // Configuration loading strategy:
    // - Load from JSON when --read-json flag present
    // - Otherwise validate and convert CLI arguments
    let info = if cli.read_json {
        read_json()
    } else {
        check_cli(&cli);
        cli_to_info(cli)
    };

    // Windows console UTF-8 encoding enforcement
    #[cfg(windows)]
    cmd_to_utf8();

    // Output validated configuration and start monitoring
    info.output_info();
    normal_loop(info, runtime_info);
}
