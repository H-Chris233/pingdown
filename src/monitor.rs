use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::ping::check_status;
use crate::runtime::{add_one, MetricEvent, Metrics};
use crate::system::{error, System};

/// Continuously monitors connectivity in regular intervals
pub fn normal_loop<S: System>(info: Config, metrics: Arc<Mutex<Metrics>>, system: &S) {
    let vec_address = &info.vec_address;
    let secs = info.secs_for_normal_loop;
    println!("Started {}sec loop...", secs);
    for i in 0.. {
        let status = check_status(vec_address, &info.strict, &metrics, system);
        if !status {
            emergency_loop(&info, &metrics, system);
            continue;
        }
        add_one(&metrics, MetricEvent::NormalLoopTimes);
        if i >= 1 {println!("Normal looped for {} times...", i);} // keep original behavior
        println!("{} secs left for the next normal loop...", secs);
        system.sleep_secs(secs);
    }
}

/// Critical failure handler activated when connectivity is lost. Implements retry mechanism and system shutdown protocol.
fn emergency_loop<S: System>(info: &Config, metrics: &Arc<Mutex<Metrics>>, system: &S) {
    let vec_address = &info.vec_address;
    let secs = info.secs_for_emergency_loop;
    let mut time_left = info.times_for_emergency_loop;
    println!("Warning!!! Connection lost!!!!");
    println!("Checking connection every {} seconds!!", secs);
    loop {
        println!("{} tries remaining...", time_left);
        let status = check_status(vec_address, &info.strict, metrics, system);
        if status {
            break;
        } else if time_left == 0 {
            system.shutdown();
            error("system shutdown failed - check permissions");
        }
        println!("{} secs left for the next check...", secs);
        add_one(metrics, MetricEvent::EmergencyLoopTimes);
        time_left -= 1;
        system.sleep_secs(secs);
    }
    println!("Reconnected!!!");
    println!("Exiting {}sec emergency loop...", secs);
}
