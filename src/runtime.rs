use std::fs;
use std::sync::{Arc, Mutex};

use crate::system::error;

/// Tracks various runtime metrics for monitoring and reporting.
#[derive(Debug)]
pub struct Metrics {
    pub total_succeeds: u64,
    pub total_failures: u64,
    pub total_normal_loop_times: u64,
    pub total_emergency_loop_times: u64,
}

/// Event types that can increment runtime metrics.
pub enum MetricEvent {
    /// Successful operation count
    Succeeds,
    /// Failed operation count
    Failures,
    /// Standard execution cycles
    NormalLoopTimes,
    /// Error recovery cycles
    EmergencyLoopTimes,
}

impl Metrics {
    /// Creates a new instance with all counters initialized to zero.
    pub fn new() -> Self {
        Self {
            total_succeeds: 0,
            total_failures: 0,
            total_normal_loop_times: 0,
            total_emergency_loop_times: 0,
        }
    }

    /// Prints current metrics to stdout in debug format.
    pub fn output(&self) { println!("{:#?}", self); }

    /// Writes metrics to file if any counter is non-zero.
    /// Creates/overwrites 'pingdown_runtime_info.txt' with debug-formatted data.
    /// Shows error message if file write fails (e.g., permission issues).
    pub fn write(&self) {
        match self {
            Metrics { total_succeeds: 0, total_failures: 0, total_normal_loop_times: 0, total_emergency_loop_times: 0 } => {}
            _ => {
                match fs::write("pingdown_runtime_info.txt", format!("{:#?}", self)) {
                    Ok(_) => {}
                    Err(err) => error(&format!("writing output file[{}], please check your permission.", err)),
                }
            }
        }
    }
}

/// Thread-safe counter increment for runtime metrics.
/// Locks the mutex and increments the specified counter by 1.
/// Terminates application on lock poisoning (unrecoverable error).
pub fn add_one(metrics: &Arc<Mutex<Metrics>>, key: MetricEvent) {
    let mut guard = match metrics.lock() {
        Ok(output) => output,
        Err(err) => error(&format!("locking value [{}]", err)),
    };
    match key {
        MetricEvent::Succeeds => guard.total_succeeds += 1,
        MetricEvent::Failures => guard.total_failures += 1,
        MetricEvent::NormalLoopTimes => guard.total_normal_loop_times += 1,
        MetricEvent::EmergencyLoopTimes => guard.total_emergency_loop_times += 1,
    }
}
