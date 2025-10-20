use std::io;
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;
use colored::Colorize;

/// Platform abstraction for shell execution, ping command construction,
/// shutdown flows, and console tweaks.
pub trait System {
    fn run_shell_command(&self, command: &str, message: Option<&str>) -> io::Result<Output>;
    fn shutdown(&self);
    fn console_setup(&self);
    fn build_ping_command(&self, ip: &str) -> String;
    fn sleep_secs(&self, secs: u64) {
        thread::sleep(Duration::from_secs(secs));
    }
}

/// Default concrete implementation that delegates to OS-specific strategies
/// via conditional compilation.
#[derive(Clone, Copy, Default)]
pub struct DefaultSystem;

impl DefaultSystem {
    pub fn new() -> Self { Self }
}

#[cfg(unix)]
impl System for DefaultSystem {
    fn run_shell_command(&self, command: &str, message: Option<&str>) -> io::Result<Output> {
        if let Some(message) = message { println!("{}", message) }
        Command::new("sh").arg("-c").arg(command).output()
    }

    fn shutdown(&self) {
        let _ = self.run_shell_command("shutdown -h now", Some("Starting shutdown..."));
        self.sleep_secs(7);
        let _ = self.run_shell_command("poweroff", None);
        self.sleep_secs(7);
        let _ = self.run_shell_command("poweroff -f", None);
        self.sleep_secs(7);
        let _ = self.run_shell_command("halt", None);
        self.sleep_secs(7);
        let _ = self.run_shell_command("init 0", None);
        self.sleep_secs(7);
        let _ = self.run_shell_command("systemctl poweroff", None);
    }

    fn console_setup(&self) { /* no-op on Unix */ }

    fn build_ping_command(&self, ip: &str) -> String {
        format!("ping -c 1 {}", ip)
    }
}

#[cfg(windows)]
impl System for DefaultSystem {
    fn run_shell_command(&self, command: &str, message: Option<&str>) -> io::Result<Output> {
        if let Some(message) = message { println!("{}", message) }
        Command::new("cmd").arg("/C").arg(command).output()
    }

    fn shutdown(&self) {
        let _ = self.run_shell_command("shutdown /s /t 0", Some("Starting shutdown..."));
    }

    fn console_setup(&self) {
        let _ = self.run_shell_command("chcp 65001", None);
    }

    fn build_ping_command(&self, ip: &str) -> String {
        format!("ping -n 1 {}", ip)
    }
}

/// Terminates program after critical errors with diagnostic information
pub fn error(message: &str) -> ! {
    eprintln!("\nAn {} occurred during {}\nif it's {} your fault, please contact {} or new an issue on https://www.github.com/H-Chris233/pingdown",
        "Error".red().bold(), message, "not".yellow().bold(), "h-chris233@outlook.com".blue());
    thread::sleep(Duration::from_secs(5));
    std::process::exit(1);
}
