#![allow(dead_code)]
#![allow(unused)]

use ping_shutdown::*;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::process::Output;
use std::env;
use std::io;

#[cfg(windows)]
const SHUTDOWN_COMMAND: &str = "shutdown /s /t 0";

#[cfg(unix)]
const SHUTDOWN_COMMAND: &str = "poweroff";

const BING: &str = "bing.com";

fn main() {
    #[cfg(windows)]
    cmd_to_utf8();
    
    println!("Started running...");
    let ip1 = "192.168.3.8";
    let ip2 = "192.168.3.9";
    loop_60sec(ip1 ,ip2);
}

fn loop_60sec(ip1:&str ,ip2:&str) {
    let secs = 60;
    println!("Started 60sec loop...");
    for i in 1.. {
        println!("60sec Looped for {} times...", i);
        let status = verify(ip1,ip2,secs);
        if status == false {
            loop_20sec(ip1 ,ip2);
            continue;
        }
        sleep(Duration::from_secs(60));
    }
}

fn get_status(ip:&str) -> String {
    let command = format!("ping {} -n 1" ,ip);
    let message = format!("Started clienting {}..." ,ip);
    let output = match run_command(&command, &message){
    Ok(output) => output,
    Err(_) => error("running command"),
    };
    
    let status = String::from_utf8_lossy(&output.stdout);
    let status =  status.to_string();
    status
}

fn check_status(ip:&str) -> bool {
    let status = get_status(ip);
    println!("Started checking {}..." ,ip);
    if status.contains("TTL") {
        println!("fine.");
        return true;
    }else{
        println!("Request timed out.");
        return false;
    }

}

fn verify(ip1:&str ,ip2:&str ,secs:i32) -> bool {
    let status1 = check_status(ip1);
    let status2 = check_status(ip2);
    println!("{} secs left for the next loop..." ,secs);
    if status1 == false && status2 == false {
        return false;
    }else{
        return true;
    }
}

fn loop_20sec(ip1:&str ,ip2:&str) {
    let secs = 20;
    let mut time_left = 3;
    println!("Warning!!! Connection lost!!!!");
    println!("Checking web connection per 20 seconds!!");
    loop{
        println!("{} times left for shutting down...", time_left);
        let status = verify(ip1,ip2,secs);
        if status == true {
            break;
        }else if time_left == 0 {
            shutdown();
        }
        sleep(Duration::from_secs(20));
        time_left -= 1;
    }
    println!("Reconnected!!!");
    println!("Exiting 20sec loop...");
}

fn shutdown() {
    let _ =run_command(SHUTDOWN_COMMAND, "Started shutting down...");
}

#[cfg(windows)]
fn run_command(command: &str, message: &str) -> io::Result<Output> {
    println!("{}", message);
    let output = Command::new("cmd")
        .arg("/C")
        .arg(command)
        .output()?;
    Ok(output)
}

#[cfg(unix)]
fn run_command(command: &str, message: &str) -> io::Result<Output> {
    println!("{}", message);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    Ok(output)
}

#[cfg(windows)]
fn cmd_to_utf8() {
    let _ = match run_command("chcp 65001", "......") {
    Ok(output) => output,
    Err(_) => error(),
    };
}

fn error(message: &str) -> ! {
    eprintln!("An error occurred when {},please send an email to h-chris233@qq.com or open a issue to help me improve, tanks!", message);
    sleep(Duration::from_secs(7));
    std::process::exit(1);
}

fn get_args() -> Vec<String> {
    let usage = "Usage:ping_shutdown -t (<secs for a normal loop>,<secs for an emergency fast loop>)[DEFAULT:(60, 20)]\n-A[DEFAULT](shutdown when all ips are unavailable)\n-O(shutdown when any ip is unavailable)\n-l <times for emergency loop>[Default:3]";
    let mut args = vec![];
    for arg in match env::args().skip(1) {
        Ok(arg) => arg,
        Err(_) => error("getting args[in function get_args]"),
    } {
        args.push(arg);
        if args.len() == 0 {
            println!("As default,will check the connection with bing.com...")
        }
    }
    args
}


fn read_args(args: Vec<String>) -> Option<String> {
    let flag: Option<&str> = None;
    let mut args_in = ArgsIn {
        and_or: None,
        ip: None,
        secs_for_loop: None,
        times_for_emergency_loop: None,
    };
    for arg in args{
        match flag {
            Some("secs_for_loop") => {},
            Some("times_for_emergency_loop") => {},
            Some("ip") => {},
            None => {},
            _ => error("matching flag[in function read_args]"),
        }
        match &arg as &str {
            "-t" => {let flag = "secs_for_loop";},
            "-A" => {args_in.and_or = Some(false);},
            "-O" => {args_in.and_or = Some(true);},
            "-l" => {let flag = "times_for_emergency_loop";},
            "-ip" => {let flag = "ip";},
            _ => error("matching arg"),
        }
        
        
        
    }
    None
}



















