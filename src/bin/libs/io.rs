use std::process::Command;
pub use std::time::Duration;
pub use std::thread;
pub use std::process::Output;
pub use std::io;

/// Unix command line execution (sh)
#[cfg(unix)]
pub fn run_command(command: &str, message: Option<&str>) -> io::Result<Output> {
    match message {
        Some(message) => println!("{}", message),
        None => {},
    }
    let output = Command::new("sh").arg("-c").arg(command).output()?;
    Ok(output)
}

/// Windows command line execution (cmd)
#[cfg(windows)]
pub fn run_command(command: &str, message: Option<&str>) -> io::Result<Output> {
    match message {
        Some(message) => println!("{}", message),
        None => {},
    }
    let output = Command::new("cmd").arg("/C").arg(command).output()?;
    Ok(output)
}

/// Unix shutdown command implementation with fallback methods
#[cfg(unix)]
pub fn shutdown() {
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

/// Windows shutdown command implementation
#[cfg(windows)]
pub fn shutdown() {
    run_command("shutdown /s /t 0", Some("Starting shutdown..."));
}

/// Configures Windows console for UTF-8 text encoding
#[cfg(windows)]
pub fn cmd_to_utf8() {
    // 65001 is the Windows code page identifier for UTF-8 encoding
    let _ = match run_command("chcp 65001", None) {
        Ok(output) => output,
        Err(_) => error("configuring console encoding [cmd_to_utf8]"),
    };
}

/// Terminates program after critical errors with diagnostic information
pub fn error(message: &str) -> ! {
    eprintln!("An error occurred during {}\nif it's not your fault, please contact h-chris233@qq.com or open an issue on https://www.github.com/H-Chris233/pingdown", message);
    sleep(5);
    std::process::exit(1);
}

/// Suspends execution for specified duration
pub fn sleep(time: u64) {
    thread::sleep(Duration::from_secs(time));
}
