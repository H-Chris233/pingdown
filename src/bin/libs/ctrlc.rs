use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, tick};
use crate::libs::output_file::RuntimeInfo;
use anyhow::{Context, Result};
use log::{debug, error, info};

pub fn ctrlc_init() -> Result<Arc<Mutex<RuntimeInfo>>> {
    // Thread-safe shared state for runtime statistics
    let runtime_info = Arc::new(Mutex::new(RuntimeInfo::new()));
    let runtime_info_clone = Arc::clone(&runtime_info);

    // Atomic flag for graceful shutdown on Ctrl-C
    let ctrlc_flag = Arc::new(AtomicBool::new(false));
    let ctrlc_clone = ctrlc_flag.clone();

    // Periodic signal check interval (80ms)
    let ticker = tick(Duration::from_millis(80));

    // Register system signal handler to set termination flag
    ctrlc::set_handler(move || {
        ctrlc_clone.store(true, Ordering::SeqCst);
    }).context("Failed to set Ctrl+C handler")?;

    // Cleanup thread: monitors termination flag and persists final state
    thread::spawn(move || {
        loop {
            select! {
                recv(ticker) -> _ => {
                    if ctrlc_flag.load(Ordering::SeqCst) { // Check termination flag every 80 millis
                        info!("Writing final results and exiting...");
                        
                        // Acquire lock with error handling
                        let output = match runtime_info_clone.lock() {
                            Ok(guard) => guard,
                            Err(err) => {
                                error!("Failed to acquire mutex lock: {:?}", err);
                                continue;
                            }
                        };
                        
                        // Output results with logging
                        debug!("Final runtime state: {:#?}", output);
                        
                        // Write to file with error handling
                        if let Err(e) = output.write() {
                            error!("Failed to write runtime info: {}", e);
                        }
                        
                        // Graceful exit after writing file
                        std::process::exit(0);
                    }
                }
            }
        }
    });
    
    Ok(runtime_info)
}
