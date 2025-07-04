
use pingdown::JsonInfo;
use crate::libs::ping::check_status;
use crate::libs::io::{sleep, shutdown};
use crate::libs::output_file::{RuntimeInfo, Info, add_one};
use std::sync::{Arc, Mutex};
use anyhow::{Context, Result};
use log::{debug, error, info, warn};

/// Continuously monitors connectivity in regular intervals
pub fn normal_loop(info: JsonInfo, runtime_info: Arc<Mutex<RuntimeInfo>>) -> Result<()> {
    let vec_address = &info.vec_address;
    let secs = info.secs_for_normal_loop;
    info!("Started {} sec normal loop...", secs);
    
    for i in 0.. {
        match check_status(vec_address, &info.strict, &runtime_info) {
            Ok(status) => {
                if !status {
                    debug!("Connection lost, entering emergency loop");
                    if let Err(e) = emergency_loop(&info, &runtime_info) {
                        error!("Emergency loop failed: {}", e);
                    }
                    continue;
                }
            },
            Err(e) => {
                error!("Connection check failed: {}", e);
                debug!("Entering emergency loop due to connection error");
                if let Err(e) = emergency_loop(&info, &runtime_info) {
                    error!("Emergency loop failed: {}", e);
                }
                continue;
            }
        }

        add_one(&runtime_info, Info::NormalLoopTimes)?;
        if i >= 1 {
            info!("Normal looped for {} times...", i + 1);
        }
        debug!("{} secs left for the next normal loop...", secs);
        sleep(secs);
    }
    
    Ok(())
}

/// Critical failure handler activated when connectivity is lost
fn emergency_loop(info: &JsonInfo, runtime_info: &Arc<Mutex<RuntimeInfo>>) -> Result<()> {
    let vec_address = &info.vec_address;
    let secs = info.secs_for_emergency_loop;
    let max_retries = info.times_for_emergency_loop;
    let mut retries_left = max_retries;
    
    warn!("Connection lost - Entering emergency loop");
    info!("Emergency loop configured for {} retries every {} seconds", max_retries, secs);

    loop {
        info!("{} retries remaining...", retries_left);
        
        match check_status(vec_address, &info.strict, runtime_info) {
            Ok(true) => {
                info!("Connection restored after {} retries!", max_retries - retries_left);
                return Ok(());
            },
            Ok(false) => {
                retries_left -= 1;
                if retries_left == 0 {
                    warn!("Emergency loop shutdown triggered - no connection");
                    match shutdown() {
                        Ok(_) => info!("System is shutting down..."),
                        Err(e) => {
                            error!("System shutdown failed: {}", e);
                            return Err(e).context("Emergency shutdown failed");
                        }
                    }
                }
            },
            Err(e) => {
                error!("Connection check error: {}", e);
                retries_left -= 1;
            }
        }
        
        add_one(runtime_info, Info::EmergencyLoopTimes)?;
        debug!("{} secs left for the next emergency check...", secs);
        sleep(secs);
    }
}
