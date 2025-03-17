use pingdown::Info;
use crate::libs::ping::check_status;
use crate::libs::io::{sleep, error, shutdown};
use crate::libs::output_file::RuntimeInfo;

/// Continuously monitors connectivity in regular intervals
pub fn normal_loop(info: &Info, runtime_info: &mut RuntimeInfo) {
    let vec_address = &info.vec_address;
    let secs = info.secs_for_normal_loop;
    println!("Started {}sec loop...", secs);
    for i in 0.. {
        let status = check_status(vec_address, &info.strict, runtime_info);
        if status == false {
            emergency_loop(info, runtime_info);
            continue;
        }
        runtime_info.total_normal_loop_times += 1;
        if i >= 1 {println!("Normal looped for {} times...", i);}
        println!("{} secs left for the next normal loop...", secs);
        println!("Total times for normal loop:{}", &runtime_info.total_normal_loop_times);
        sleep(secs);
    }
}

/// Critical failure handler activated when connectivity is lost. Implements retry mechanism and system shutdown protocol.
fn emergency_loop(info: &Info, runtime_info: &mut RuntimeInfo) {
    let vec_address = &info.vec_address;
    let secs = info.secs_for_emergency_loop;
    let mut time_left = info.times_for_emergency_loop;
    println!("Warning!!! Connection lost!!!!");
    println!("Checking connection every {} seconds!!", secs);
    loop {
        
        println!("{} tries remaining...", time_left);
        let status = check_status(vec_address, &info.strict, runtime_info);
        if status == true {
            break;
        } else if time_left <= 0 {
            shutdown();
            error("system shutdown failed - check permissions"); // System still running indicates permission issues
        }
        println!("{} secs left for the next check...", secs);
        runtime_info.total_emergency_loop_times += 1;
        time_left -= 1;
        println!("Total times for emergency loop:{}", &runtime_info.total_emergency_loop_times);
        sleep(secs);
    }
    println!("Reconnected!!!");
    println!("Exiting {}sec emergency loop...", secs);
}









