//! Runtime statistics tracking for the application.

use std::fs;
use crate::libs::io::error;
use serde::Serialize;
use std::sync:: {
    Arc,
    Mutex
};

/// Tracks various runtime metrics for monitoring and reporting.
/// Serialized with aliases for JSON field naming conventions.
#[derive(Debug, Serialize)]
pub struct RuntimeInfo {
    #[serde(alias = "total-succeeds")]
    pub total_succeeds: u64,
    #[serde(alias = "total-failures")]
    pub total_failures: u64,
    #[serde(alias = "total-normal-loop-times")]
    pub total_normal_loop_times: u64,
    #[serde(alias = "total-emergency-loop-times")]
    pub total_emergency_loop_times: u64,
}

/// Event types that can increment runtime metrics.
pub enum Info {
    /// Successful operation count
    Succeeds,
    /// Failed operation count
    Failures,
    /// Standard execution cycles
    NormalLoopTimes,
    /// Error recovery cycles
    EmergencyLoopTimes,
}

impl RuntimeInfo {
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
    pub fn output(&self) {
        println!("{:#?}", self);
    }

    /// Writes metrics to file if any counter is non-zero.
    /// Creates/overwrites 'pingdown_runtime_info.txt' with debug-formatted data.
    /// Shows error message if file write fails (e.g., permission issues).
    pub fn write(&self) {
        // Only write if at least one metric has non-zero value
        match self {
            RuntimeInfo {
                total_succeeds: 0,
                total_failures: 0,
                total_normal_loop_times: 0,
                total_emergency_loop_times: 0,
            } => {}
            _ => {
                match fs::write("pingdown_runtime_info.txt", &format!("{:#?}", self)) {
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
pub fn add_one(runtime_info: &Arc<Mutex<RuntimeInfo>>, key: Info) {
    // Lock the mutex (may panic on poisoning)
    let mut runtime_info = match runtime_info.lock() {
        Ok(output) => output,
        Err(err) => error(&format!("locking value [{}]", err)),
    };

    // Increment appropriate counter
    match key {
        Info::Succeeds => runtime_info.total_succeeds += 1,
        Info::Failures => runtime_info.total_failures += 1,
        Info::NormalLoopTimes => runtime_info.total_normal_loop_times += 1,
        Info::EmergencyLoopTimes => runtime_info.total_emergency_loop_times += 1,
    }
}