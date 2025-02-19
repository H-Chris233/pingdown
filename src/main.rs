use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::process::Output;

#[cfg(windows)]
const SHUTDOWN_COMMAND: &str = "shutdown /s /t 0";

#[cfg(unix)]
const SHUTDOWN_COMMAND: &str = "poweroff";

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
        println!("60sec Looped for {i} times...");
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
    let output = run_command(&command, &message);
    
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
    loop{
        println!("{time_left} times left for shutting down...");
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
    run_command(SHUTDOWN_COMMAND, "Started shutting down...");
}

#[cfg(windows)]
fn run_command(command: &str, message: &str) -> Output {
    println!("{}", message);
    let output = Command::new("cmd")
        .arg("/C")
        .arg(command)
        .output()
        .expect("I/O ERROR!!!");
    output
}

#[cfg(unix)]
fn run_command(command: &str, message: &str) -> Output {
    println!("{}", message);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("I/O ERROR!!!");
    output
}

#[cfg(windows)]
fn cmd_to_utf8() {
    let _command = Command::new("cmd")
        .arg("/C")
        .arg("chcp 65001")
        .output()
        .expect("I/O ERROR!!!");
}