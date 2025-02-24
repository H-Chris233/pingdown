#![allow(dead_code)]
#![allow(unused)]

use std::process::Command;
use std::process::Output;
use std::time::Duration;
use ping_shutdown::*;
use std::thread;
use std::io;

fn main() {
    let args_in = ArgsIn::parse();

    #[cfg(windows)]
    cmd_to_utf8();
    
    
    
    
    println!("Started running...");
    let ip = &args_in.ip.clone();
    normal_loop(ip, &args_in);
}

fn normal_loop(ip: &str, args_in: &ArgsIn) {
    let secs: u64 = match args_in.secs_for_normal_loop.parse() {
        Ok(secs) => secs,
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a number[in function normal_loop]")
        }
    };
    println!("Started {}sec loop...", secs);
    for i in 1.. {
        let status = get_status(ip);
        if status == false {
            emergency_loop(ip, args_in);
            continue;
        }
        println!("Normal looped for {} times...", i);
        println!("{} secs left for the next normal loop...", secs);
        sleep(secs);
    }
}

fn get_status(ip: &str) -> bool {
    let command = format!("ping -c 1 {}", ip);
    let message = format!("Started clienting {}...", ip);
    let output = match run_command(&command, Some(&message)) {
        Ok(output) => output,
        Err(_) => error("running command[in function get_status]"),
    };
    let status = String::from_utf8_lossy(&output.stdout);
    let status = status.to_string();
    println!("Started checking {}...", ip);
    if status.contains("TTL") || status.contains("ttl") {
         println!("fine.");
         true
    } else {
          println!("Request timed out.");
          false
    }
}

fn check_status(ip: &str, args_in: &ArgsIn) -> bool {
    let status = get_status(ip);
    let and_or: bool = match args_in.and_or.as_str() {
        "" => true,
        "None" => false,
        _ => error("reading a bad argument"),
    };
    
    true
}

fn emergency_loop(ip: &str, args_in: &ArgsIn) {
    let secs: u64 = match args_in.secs_for_emergency_loop.parse() {
        Ok(secs) => secs,
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a usable number[in function emergency_loop]");
        }
    };
    let mut time_left: usize = match args_in.times_for_emergency_loop.parse() {
        Ok(time_left) => time_left,
        Err(_) => {
            println!("Please check your input.");
            error("turning input to a usable number[in function emergency_loop]");
        }
    };
    println!("Warning!!! Connection lost!!!!");
    println!("Checking web connection per {} seconds!!", secs);
    loop {
        println!("{} times left for shutting down...", time_left);
        let status = get_status(ip);
        if status == true {
            break;
        } else if time_left <= 0 {
            shutdown();
            error("shutting down[permission denied]");
        }
        println!("{} secs left for the next loop...", secs);
        sleep(secs);
        time_left -= 1;
    }
    println!("Reconnected!!!");
    println!("Exiting {}sec loop...", secs);
}

#[cfg(windows)]
fn shutdown() {
    run_command("shutdown /s /t 0", Some("Started shutting down..."));
}

#[cfg(unix)]
fn shutdown() {
    run_command("shutdown -h now", Some("Started shutting down..."));
    sleep(7);
    run_command("poweroff", None);
    sleep(7);
    run_command("poweroff -f", None);
    sleep(7);
    run_command("halt", None);
    sleep(7);
    run_command("init", None);
    sleep(7);
    run_command("systemctl poweroff", None);
}

#[cfg(windows)]
fn run_command(command: &str, message: Option<&str>) -> io::Result<Output> {
    match message {
        Some(message) => println!("{}", message),
        None => {},
    }
    let output = Command::new("cmd").arg("/C").arg(command).output()?;
    Ok(output)
}

#[cfg(unix)]
fn run_command(command: &str, message: Option<&str>) -> io::Result<Output> {
    match message {
        Some(message) => println!("{}", message),
        None => {},
    }
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    Ok(output)
}

#[cfg(windows)]
fn cmd_to_utf8() {
    let _ = match run_command("chcp 65001", None) {
        Ok(output) => output,
        Err(_) => error("turning cmd to UTF-8,[in function cmd_to_utf8]"),
    };
}

fn error(message: &str) -> ! {
    eprintln!("Sorry, an error occurred when {},please send an email to h-chris233@qq.com or open a issue to help me improve, thanks!", message);
    sleep(7);
    std::process::exit(1);
}

fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}
