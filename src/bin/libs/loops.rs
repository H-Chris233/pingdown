use pingdown::{Info, Output};
use crate::libs::ping::check_status;
use crate::libs::io::{sleep, error, shutdown};

/// Continuously monitors connectivity in regular intervals
pub fn normal_loop(info: &Info, output: &mut Output) {
    let vec_address = &info.vec_address;
    let secs = info.secs_for_normal_loop;
    println!("Started {}sec loop...", secs);
    for i in 1.. {
        let status = check_status(vec_address, &info.strict, output);
        if status == false {
            emergency_loop(info, output);
            continue;
        }
        output.total_normal_loop_times += 1;
        println!("Normal looped for {} times...", i);
        println!("{} secs left for the next normal loop...", secs);
        sleep(secs);
    }
}

/// Critical failure handler activated when connectivity is lost. Implements retry mechanism and system shutdown protocol.
fn emergency_loop(info: &Info, output: &mut Output) {
    let vec_address = &info.vec_address;
    let secs = info.secs_for_emergency_loop;
    let mut time_left = info.times_for_emergency_loop;
    println!("Warning!!! Connection lost!!!!");
    println!("Checking connection every {} seconds!!", secs);
    loop {
        println!("{} tries remaining...", time_left);
        let status = check_status(vec_address, &info.strict, output);
        if status == true {
            break;
        } else if time_left <= 0 {
            shutdown();
            error("system shutdown failed - check permissions"); // System still running indicates permission issues
        }
        println!("{} secs left for the next check...", secs);
        sleep(secs);
        time_left -= 1;
    }
    println!("Reconnected!!!");
    println!("Exiting {}sec emergency loop...", secs);
}
