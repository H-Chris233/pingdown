use std::process::Command;
use std::process::Output;
use std::time::Duration;
use pingdown::*;
use std::thread;
use std::io;

/// Handles argument processing. Adjusts terminal encoding on Windows. Acts as the program entry point.
fn main() {
    let cli = Cli::parse();
    #[cfg(windows)]
    cmd_to_utf8();
    
    if cli.vec_ip.is_empty()  {
        println!("Please input at least one ip or website.\nYou can also use -h or --help to get help.");
        sleep(4);
        std::process::exit(0);
    }
    output_message(&cli);
    normal_loop(&cli.vec_ip, &cli);
}

/// Continuously monitors connectivity in regular intervals
fn normal_loop(vec_ip: &Vec<String>, cli: &Cli) {
    let secs: u64 = match cli.secs_for_normal_loop.parse() {
        Ok(secs) => secs, // Parses user-defined interval duration
        Err(_) => {
            println!("Please check your input.");
            error("parsing interval duration [normal_loop context]")
        }
    };
    println!("Started {}sec loop...", secs);
    for i in 1.. {
        let status = check_status(vec_ip, cli);
        if status == false {
            emergency_loop(vec_ip, cli);
            continue;
        }
        println!("Normal looped for {} times...", i);
        println!("{} secs left for the next normal loop...", secs);
        sleep(secs);
    }
}

/// Tests connectivity to a single target using system ping command
fn get_status(ip: &str) -> bool {
    let command = format!("ping -c 1 {}", ip);
    let message = format!("Started pinging {}...", ip);
    let output = match run_command(&command, Some(&message)) {
        Ok(output) => output, // Gets command output
        Err(_) => error("executing command[in get_status]"),
    };
    let status = String::from_utf8_lossy(&output.stdout).to_string(); // Converts byte stream to UTF-8 string with invalid character handling
    println!("Started checking {}...", ip);
    if status.contains("TTL") || status.contains("ttl") { // Checks for TTL presence to determine success
         println!("Success.");
         true
    } else {
         println!("Request timed out.");
         false // indicates connection failure state
    }
}

/// Evaluates connectivity status across multiple targets according to monitoring mode
fn check_status(vec_ip: &Vec<String>, cli: &Cli) -> bool {
    let mut status_vec: Vec<bool> = vec![];
    for ip in vec_ip {
        let status = get_status(ip);
        status_vec.push(status);
    }
    let status = match cli.strict {
        false => {
            match status_vec.contains(&true) { // Default mode: any successful connection passes
                true => true,
                false => false,
            }
        },
        true => {
            match status_vec.contains(&false) { // Strict mode: requires all connections to succeed
                true => false,
                false => true,
            }
        },
    };
    status
}

/// Critical failure handler activated when connectivity is lost. Implements retry mechanism and system shutdown protocol.
fn emergency_loop(vec_ip: &Vec<String>, cli: &Cli) {
    let secs: u64 = match cli.secs_for_emergency_loop.parse() {
        Ok(secs) => secs, // Parses emergency retry interval duration
        Err(_) => {
            println!("Please check your input.");
            error("parsing emergency interval [emergency_loop]");
        }
    };
    let mut time_left: usize = match cli.times_for_emergency_loop.parse() {
        Ok(time_left) => time_left,
        Err(_) => {
            println!("Please check your input.");
            error("converting input to number[in emergency_loop]");
        }
    };
    println!("Warning!!! Connection lost!!!!");
    println!("Checking connection every {} seconds!!", secs);
    loop {
        println!("{} tries remaining...", time_left);
        let status = check_status(vec_ip, cli);
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

/// Windows shutdown command implementation
#[cfg(windows)]
fn shutdown() {
    run_command("shutdown /s /t 0", Some("Starting shutdown..."));
}

/// Unix shutdown command implementation with fallback methods
#[cfg(unix)]
fn shutdown() {
    let _ = run_command("shutdown -h now", Some("Starting shutdown..."));
    sleep(7); // Tries multiple shutdown commands with 7-second delays
    let _ = run_command("poweroff", None);
    sleep(7);
    let _ = run_command("poweroff -f", None);
    sleep(7);
    let _ = run_command("halt", None);
    sleep(7);
    let _ = run_command("init 0", None);
    sleep(7);
    let _ = run_command("systemctl poweroff", None);
}

/// Windows command line execution (cmd)
#[cfg(windows)]
fn run_command(command: &str, message: Option<&str>) -> io::Result<Output> {
    match message {
        Some(message) => println!("{}", message),
        None => {},
    }
    let output = Command::new("cmd").arg("/C").arg(command).output()?;
    Ok(output)
}

/// Unix command line execution (sh)
#[cfg(unix)]
fn run_command(command: &str, message: Option<&str>) -> io::Result<Output> {
    match message {
        Some(message) => println!("{}", message),
        None => {},
    }
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    Ok(output)
}

/// Configures Windows console for UTF-8 text encoding
#[cfg(windows)]
fn cmd_to_utf8() {
    // 65001 is the Windows code page identifier for UTF-8 encoding
    let _ = match run_command("chcp 65001", None) {
        Ok(output) => output,
        Err(_) => error("configuring console encoding [cmd_to_utf8]"),
    };
}

/// Terminates program after critical errors with diagnostic information
fn error(message: &str) -> ! {
    eprintln!("Error occurred during {}, please contact h-chris233@qq.com or open an issue.", message);
    sleep(7);
    std::process::exit(1);
}

/// Suspends execution for specified duration
fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}

/// Output configuration.
fn output_message(cli: &Cli) {
    println!("{:#?}", cli);
    println!("Started running...");
}
