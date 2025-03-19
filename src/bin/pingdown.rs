//! Main entry point and core logic for the pingdown network monitoring tool.
//! Handles signal catching, CLI parsing, and kicking off the main monitoring loop.

#![allow(dead_code)]
#![allow(unused)]

mod libs;

use crate::libs::check_input::check_cli;
use crate::libs::loops::normal_loop;
use crate::libs::struct_info::*;
use crate::libs::output_file::*;
use crate::libs::io::error;
use pingdown::Cli;
use clap::Parser;

// Signal handling utilities
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, tick};

/// Main orchestrator that sets up signal handlers and kicks off monitoring
fn main() {
    // Shared state for tracking program statistics
    let runtime_info = Arc::new(Mutex::new(RuntimeInfo::new()));
    let runtime_info_clone = Arc::clone(&runtime_info);
    
    // Flag for clean shutdown on Ctrl-C
    let ctrlc_flag = Arc::new(AtomicBool::new(false));
    let ctrlc_clone = ctrlc_flag.clone();
    
    // Ticker for checking shutdown signals periodically
    let ticker = tick(Duration::from_millis(80));

    // Handle Ctrl-C by setting flag and writing final output
    ctrlc::set_handler(move || {
        ctrlc_clone.store(true, Ordering::SeqCst);
    }).unwrap_or_else(|err| error(&format!("setting ctrl+c handler: {}", err)));

    // Spawn a dedicated thread for shutdown cleanup
    thread::spawn(move || {
        loop {
            select! {
                recv(ticker) -> _ => {
                    // When Ctrl-C detected, save results and bail
                    if ctrlc_flag.load(Ordering::SeqCst) {
                        println!("Writing final results and exiting...");
                        let output = runtime_info_clone.lock()
                            .unwrap_or_else(|err| error(&format!("locking runtime info: {}", err)));
                        output.write();
                        std::process::exit(0);
                    }
                }
            }
        }
    });
    
    // Start the actual monitoring work
    entry(runtime_info);
}

/// Entry point for network monitoring logic
/// Handles CLI/config parsing and terminal setup
fn entry(runtime_info: Arc<Mutex<RuntimeInfo>>) {
    // Parse command-line arguments
    let cli = Cli::parse();
    
    // Load config from JSON or CLI arguments
    let info = if cli.read_json {
        read_json()
    } else {
        check_cli(&cli);
        cli_to_info(cli)
    };

    // Fix Windows terminal encoding quirks
    #[cfg(windows)]
    cmd_to_utf8();

    // Show config summary and start monitoring
    info.output_info();
    normal_loop(&info, runtime_info);
}

