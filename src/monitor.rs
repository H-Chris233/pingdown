use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use colored::Colorize;

use crate::config::Config;
use crate::ping::check_status;
use crate::runtime::{add_one, MetricEvent, Metrics};
use crate::system::{error, System};

/// Continuously monitors connectivity in regular intervals
pub fn normal_loop<S: System>(info: Config, metrics: Arc<Mutex<Metrics>>, system: &S) {
    let secs = info.secs_for_normal_loop;
    println!("{} {}sec loop...", "[NORMAL]".bold().green(), secs);
    for i in 0.. {
        let (status, succeeds, failures) = check_status(&info, &metrics, system);
        if !status {
            emergency_loop(&info, &metrics, system);
            continue;
        }
        add_one(&metrics, MetricEvent::NormalLoopTimes);
        if i >= 1 && info.verbose > 0 {
            println!("{} Normal loop {}", "[NORMAL]".bold().green(), i);
        }
        println!(
            "{} {} | up: {} | down: {} | next: {}s",
            "[NORMAL]".bold().green(),
            "OK".bold().green(),
            succeeds,
            failures,
            secs
        );
        sleep_with_progress(secs, info.progress, "[NORMAL]");
    }
}

/// Critical failure handler activated when connectivity is lost. Implements retry mechanism and system shutdown protocol.
fn emergency_loop<S: System>(info: &Config, metrics: &Arc<Mutex<Metrics>>, system: &S) {
    let secs = info.secs_for_emergency_loop;
    let mut time_left = info.times_for_emergency_loop;
    println!(
        "{} Connection lost. Entering emergency loop ({}s interval, {} tries).",
        "[EMERGENCY]".bold().red(),
        secs,
        time_left
    );
    loop {
        println!("{} {} tries remaining...", "[EMERGENCY]".bold().red(), time_left);
        let (status, succeeds, failures) = check_status(info, metrics, system);
        if status {
            println!("{} Reconnected.", "[EMERGENCY]".bold().green());
            break;
        } else if time_left == 0 {
            println!("{} Exceeded maximum retries. Shutting down...", "[EMERGENCY]".bold().red());
            system.shutdown();
            error("system shutdown failed - check permissions");
        }
        println!(
            "{} {} | up: {} | down: {} | next: {}s",
            "[EMERGENCY]".bold().red(),
            "DOWN".bold().red(),
            succeeds,
            failures,
            secs
        );
        add_one(metrics, MetricEvent::EmergencyLoopTimes);
        time_left -= 1;
        sleep_with_progress(secs, info.progress, "[EMERGENCY]");
    }
    println!("{} Exiting {}sec emergency loop...", "[EMERGENCY]".bold().green(), secs);
}

fn sleep_with_progress(secs: u64, progress: bool, prefix: &str) {
    if !progress {
        println!("{} {} secs left for the next check...", prefix, secs);
        thread::sleep(Duration::from_secs(secs));
        return;
    }
    let spinner = ["-", "\\", "|", "/"]; // simple spinner
    let mut elapsed = 0u64;
    let mut i = 0usize;
    println!("{} Waiting {}s...", prefix, secs);
    while elapsed < secs {
        print!("\r{} {} {}s", prefix, spinner[i % spinner.len()], secs - elapsed);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(200));
        i += 1;
        if i % 5 == 0 {
            elapsed += 1; // advance roughly 1s per 5 ticks of 200ms
        }
    }
    print!("\r{} Done.            \n", prefix);
    let _ = io::stdout().flush();
}

#[cfg(test)]
pub fn test_emergency_loop<S: System>(info: &Config, metrics: &Arc<Mutex<Metrics>>, system: &S) {
    emergency_loop(info, metrics, system);
}
