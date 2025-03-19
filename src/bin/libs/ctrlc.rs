use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam_channel::{select, tick};
use crate::libs::output_file::RuntimeInfo;
use crate::libs::io::error;

pub fn ctrlc_init() -> Arc<Mutex<RuntimeInfo>> {
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
    }).unwrap_or_else(|err| error(&format!("setting ctrl+c handler: {}", err)));

    // Cleanup thread: monitors termination flag and persists final state
    thread::spawn(move || {
        loop {
            select! {
                recv(ticker) -> _ => {
                    // Check termination flag every tick interval
                    if ctrlc_flag.load(Ordering::SeqCst) {
                        println!("Writing final results and exiting...");
                        
                        // Safely acquire lock for final output operations
                        let output = runtime_info_clone.lock()
                        .unwrap_or_else(|err| error(&format!("locking runtime info: {}", err)));
                        
                        // Execute final persistence routines
                        output.output();
                        output.write();
                        
                        // Force exit after cleanup
                        std::process::exit(0);
                    }
                }
            }
        }
    });
    runtime_info
}
