use std::collections::HashMap;
use std::io;
use std::process::Output;
use std::sync::{Arc, Mutex};

use pingdown::system::System;

#[derive(Clone, Default)]
pub struct StubSystem {
    // For each IP, a sequence of boolean results to return across calls
    pub responses: Arc<Mutex<HashMap<String, Vec<bool>>>>,
    pub shutdown_calls: Arc<Mutex<u64>>,
}

impl StubSystem {
    pub fn new() -> Self { Self::default() }

    pub fn with_static(map: &[(&str, bool)]) -> Self {
        let mut responses = HashMap::new();
        for (ip, ok) in map {
            responses.insert((*ip).to_string(), vec![*ok]);
        }
        Self { responses: Arc::new(Mutex::new(responses)), shutdown_calls: Arc::new(Mutex::new(0)) }
    }

    pub fn push_sequence(&self, ip: &str, seq: Vec<bool>) {
        let mut guard = self.responses.lock().unwrap();
        guard.insert(ip.to_string(), seq);
    }

    pub fn take_shutdowns(&self) -> u64 { *self.shutdown_calls.lock().unwrap() }
}

#[cfg(unix)]
fn success_status() -> std::process::ExitStatus {
    use std::os::unix::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(0)
}

#[cfg(windows)]
fn success_status() -> std::process::ExitStatus {
    use std::os::windows::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(0)
}

fn make_output(stdout: &str) -> io::Result<Output> {
    Ok(Output { status: success_status(), stdout: stdout.as_bytes().to_vec(), stderr: Vec::new() })
}

impl System for StubSystem {
    fn run_shell_command(&self, command: &str, _message: Option<&str>) -> io::Result<Output> {
        // Assume command ends with the IP/host
        let ip = command.split_whitespace().last().unwrap_or("").to_string();
        let mut guard = self.responses.lock().unwrap();
        let seq = guard.entry(ip).or_insert_with(|| vec![false]);
        let result = if seq.is_empty() { false } else { seq.remove(0) };
        if result {
            make_output("Reply from 1.1.1.1: bytes=32 time=1ms TTL=64")
        } else {
            make_output("Request timed out.")
        }
    }

    fn shutdown(&self) {
        let mut g = self.shutdown_calls.lock().unwrap();
        *g += 1;
    }

    fn console_setup(&self) { /* no-op */ }

    fn build_ping_command(&self, ip: &str) -> String { format!("ping -c 1 {}", ip) }
}
