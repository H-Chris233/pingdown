#![allow(dead_code)]
#![allow(unused)]

use ping_shutdown::*;
use std::io;
use std::process::Command;
use std::process::Output;
use std::thread::sleep;
use std::time::Duration;

#[cfg(windows)]
const SHUTDOWN_COMMAND: &str = "shutdown /s /t 0";

#[cfg(unix)]
const SHUTDOWN_COMMAND: &str = "poweroff";

fn main() {
    let args_in = ArgsIn::parse();

    #[cfg(windows)]
    cmd_to_utf8();

    println!("Started running...");
    let ip = &args_in.ip.clone();
    //let normal_secs = &args_in.secs_for_normal_loop;
    //let emergency_secs = &args_in.secs_for_emergency_loop;
    normal_loop(ip, &args_in);
}

fn normal_loop(ip: &str, args_in: &ArgsIn) {
    let secs: u64 = match &args_in.secs_for_normal_loop.parse() {
        Ok(secs) => *secs,
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a number[in function normal_loop]")
        }
    };
    println!("Started {}sec loop...", secs);
    for i in 1.. {
        let status = check_status(ip);
        if status == false {
            emergency_loop(ip, &args_in);
            continue;
        }
        println!("Normal looped for {} times...", i);
        println!("{} secs left for the next normal loop...", secs);
        sleep(Duration::from_secs(secs));
    }
}

fn get_status(ip: &str) -> String {
    let command = format!("ping {} -n 1", ip);
    let message = format!("Started clienting {}...", ip);
    let output = match run_command(&command, &message) {
        Ok(output) => output,
        Err(_) => error("running command[in function get_status]"),
    };

    let status = String::from_utf8_lossy(&output.stdout);
    let status = status.to_string();
    status
}

fn check_status(ip: &str) -> bool {
    let status = get_status(ip);
    println!("Started checking {}...", ip);
    if status.contains("TTL") {
        println!("fine.");
        return true;
    } else {
        println!("Request timed out.");
        return false;
    }
}

fn emergency_loop(ip: &str, args_in: &ArgsIn) {
    let secs: u64 = match &args_in.secs_for_emergency_loop.parse() {
        Ok(secs) => *secs,
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a usable number[in function emergency_loop]");
        }
    };
    let mut time_left: usize = match &args_in.times_for_emergency_loop.clone().parse() {
        Ok(time_left) => *time_left,
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a usable number[in function emergency_loop]");
        }
    };
    println!("Warning!!! Connection lost!!!!");
    println!("Checking web connection per {} seconds!!", secs);
    loop {
        println!("{} times left for shutting down...", time_left);
        let status = check_status(ip);
        if status == true {
            break;
        } else if time_left <= 0 {
            shutdown();
        }
        println!("{} secs left for the next loop...", secs);
        sleep(Duration::from_secs(secs));
        time_left -= 1;
    }
    println!("Reconnected!!!");
    println!("Exiting {}sec loop...", secs);
}

fn shutdown() {
    let _ = run_command(SHUTDOWN_COMMAND, "Started shutting down...");
}

#[cfg(windows)]
fn run_command(command: &str, message: &str) -> io::Result<Output> {
    println!("{}", message);
    let output = Command::new("cmd").arg("/C").arg(command).output()?;
    Ok(output)
}

#[cfg(unix)]
fn run_command(command: &str, message: &str) -> io::Result<Output> {
    println!("{}", message);
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    Ok(output)
}

#[cfg(windows)]
fn cmd_to_utf8() {
    let _ = match run_command("chcp 65001", "......") {
        Ok(output) => output,
        Err(_) => error("turning cmd to UTF-8,[in function cmd_to_utf8]"),
    };
}

fn error(message: &str) -> ! {
    eprintln!("An error occurred when {},please send an email to h-chris233@qq.com or open a issue to help me improve, tanks!", message);
    sleep(Duration::from_secs(7));
    std::process::exit(1);
}
