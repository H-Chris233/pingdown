use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crossbeam_channel::{select, tick};

use crate::runtime::Metrics;
use crate::system::error;

/// Install Ctrl-C handler and background watcher that flushes runtime metrics
/// and exits the process gracefully.
pub fn install_ctrlc_handler(ctrlc_flag: Arc<AtomicBool>, metrics: Arc<Mutex<Metrics>>) {
    let ctrlc_clone = ctrlc_flag.clone();
    let metrics_clone = Arc::clone(&metrics);

    let ticker = tick(Duration::from_millis(80));

    ctrlc::set_handler(move || {
        ctrlc_clone.store(true, Ordering::SeqCst);
    }).unwrap_or_else(|err| error(&format!("setting ctrl+c handler: {}", err)));

    std::thread::spawn(move || {
        loop {
            select! {
                recv(ticker) -> _ => {
                    if ctrlc_flag.load(Ordering::SeqCst) {
                        println!("Writing final results and exiting...");
                        let output = metrics_clone.lock()
                            .unwrap_or_else(|err| error(&format!("locking runtime info: {}", err)));
                        output.output();
                        output.write();
                        std::process::exit(0);
                    }
                }
            }
        }
    });
}
