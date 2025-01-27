use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Started running...");
    let ip1 = "192.168.3.8";
    let ip2 = "192.168.3.9";
    loop_60sec(ip1 ,ip2);

}

fn loop_60sec(ip1:&str ,ip2:&str) {
    println!("Started 60sec loop...");
    let mut i:i64 = 0;
    loop{
        println!("Looped for {i} times...");
        let status = verify(ip1,ip2);
        if status == false {
            loop_20sec(ip1 ,ip2);
            continue;
        }
        sleep(Duration::from_secs(60));
        i += 1;
    }
}

fn get_status(ip:&str) -> String {
    println!("Started clienting...");
    let command = Command::new("cmd")
        .arg("/C")
        .arg(format!("ping {} -n 1" ,ip))
        .output()
        .expect("I/O ERROR!!!");
    let status = String::from_utf8_lossy(&command.stdout);
    let status =  status.to_string();
    status
}

fn patch_status(status:&str) -> &str {
    println!("Started patching...");
    let status = get_status(status);
    if status.contains("无法访问目标主机") {
        println!("Request timed out.");
        return "Request timed out.";
    }else{
        println!("...");
        return "fine."
    }

}

fn verify(ip1:&str,ip2:&str) -> bool {
    println!("Started verifying...");
    let status1 = patch_status(ip1);
    let status2 = patch_status(ip2);
    if status1 == "Request timed out." && status2 == "Request timed out." {
        return false;
    }else{
        return true;
    }
}

fn loop_20sec(ip1:&str ,ip2:&str) {
    let mut time_left = 3;
    println!("Warning!!! Connection lost!!!!");
    loop{
        println!("{time_left} times left for shutting down...");
        let status = verify(ip1,ip2);
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

