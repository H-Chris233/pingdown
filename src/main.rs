use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    cmd_to_utf8();
    println!("Started running...");
    let ip1 = "192.168.3.8";
    let ip2 = "192.168.3.9";
    loop_60sec(ip1 ,ip2);

}

fn loop_60sec(ip1:&str ,ip2:&str) {
    let secs = 60;
    println!("Started 60sec loop...");
    let mut i:i64 = 0;
    loop{
        println!("Looped for {i} times...");
        let status = verify(ip1,ip2,secs);
        if status == false {
            loop_20sec(ip1 ,ip2);
            continue;
        }
        sleep(Duration::from_secs(60));
        i += 1;
    }
}

fn get_status(ip:&str) -> String {
    println!("Started clienting {}..." ,ip);
    let output = Command::new("cmd")
        .arg("/C")
        .arg(format!("ping {} -n 1" ,ip))
        .output()
        .expect("I/O ERROR!!!");
    let status = String::from_utf8_lossy(&output.stdout);
    let status =  status.to_string();
    status
}

fn patch_status(ip:&str) -> bool {
    let status = get_status(ip);
    println!("Started patching {}..." ,ip);
    if status.contains("TTL") {
        println!("fine.");
        return true;
    }else{
        println!("Request timed out.");
        return false;
    }

}

fn verify(ip1:&str ,ip2:&str ,secs:i32) -> bool {
    let status1 = patch_status(ip1);
    let status2 = patch_status(ip2);
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
    println!("Started shutting down...");
    let _command = Command::new("cmd")
        .arg("/C")
        .arg("shutdown /s /t 0")
        .output()
        .expect("I/O ERROR!!!");
}


fn cmd_to_utf8() {
    let _command = Command::new("cmd")
        .arg("/C")
        .arg("chcp 65001")
        .output()
        .expect("I/O ERROR!!!");
}