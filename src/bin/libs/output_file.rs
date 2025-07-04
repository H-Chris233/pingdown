
use std::fs;
use std::sync::{Arc, Mutex};
use anyhow::{Context, Result};
use log::{debug, info};

/// Tracks various runtime metrics for monitoring and reporting.
#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub total_succeeds: u64,
    pub total_failures: u64,
    pub total_normal_loop_times: u64,
    pub total_emergency_loop_times: u64,
}

/// Event types that can increment runtime metrics.
#[derive(Debug, Clone, Copy)]
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
        info!("{:#?}", self);
    }

    /// Writes metrics to file if any counter is non-zero.
    /// Creates/overwrites 'pingdown_runtime_info.txt' with debug-formatted data.
    pub fn write(&self) -> Result<()> {
        // Only write if at least one metric has non-zero value
        if self.total_succeeds == 0 
            && self.total_failures == 0 
            && self.total_normal_loop_times == 0 
            && self.total_emergency_loop_times == 0 {
            
            debug!("No metrics to write - all counters are zero");
            return Ok(());
        }

        let file_path = "pingdown_runtime_info.txt";
        fs::write(file_path, format!("{:#?}", self))
            .with_context(|| format!("Failed to write runtime info to file: {}", file_path))?;
        
        debug!("Successfully wrote runtime metrics to {}", file_path);
        Ok(())
    }
}

/// Thread-safe counter increment for runtime metrics.
/// Locks the mutex and increments the specified counter by 1.
/// Returns error if mutex lock fails (unrecoverable error).
pub fn add_one(runtime_info: &Arc<Mutex<RuntimeInfo>>, key: Info) -> Result<()> {
    let mut guard = runtime_info.lock()
        .map_err(|e| anyhow::anyhow!("Mutex lock failed: {:?}", e))?;

    match key {
        Info::Succeeds => guard.total_succeeds += 1,
        Info::Failures => guard.total_failures += 1,
        Info::NormalLoopTimes => guard.total_normal_loop_times += 1,
        Info::EmergencyLoopTimes => guard.total_emergency_loop_times += 1,
    }

    Ok(())
}
