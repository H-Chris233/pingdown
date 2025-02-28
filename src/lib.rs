pub use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "pingdown")]
#[command(author = "H-Chris233")]
#[command(version = "1.3.3")]
pub struct Cli {
    /// Target IP address(es) or domain name(s) to check
    pub vec_address: Vec<String>,
    /// Enable strict verification mode
    /// 
    /// In strict mode, all targets must be reachable for the check to be considered successful.
    /// By default, the check succeeds if any target is reachable.
    #[arg(short, long)]
    pub strict: bool,
    /// Enable JSON reading mode
    ///
    /// In JSON reading mode, it will read a file named "config.json" in the current dictionary.
    #[arg(short, long)]
    pub read_json: bool,
    /// Interval (in seconds) between regular checks
    #[arg(short = 'n', default_value = "60")]
    pub secs_for_normal_loop: String,
    /// Interval (in seconds) between emergency retries
    #[arg(short = 'e', default_value = "20")]
    pub secs_for_emergency_loop: String,
    /// Maximum number of emergency retry attempts
    #[arg(short, default_value = "3")]
    pub times_for_emergency_loop: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    #[serde(alias = "address")]
    pub vec_address: Vec<String>,
    #[serde(default)]
    pub strict: bool,
    #[serde(alias = "secs-for-normal-loop", default = "default_60")]
    pub secs_for_normal_loop: u64,
    #[serde(alias = "secs-for-emergency-loop", default = "default_20")]
    pub secs_for_emergency_loop: u64,
    #[serde(alias = "times-for-emergency-loop", default = "default_3")]
    pub times_for_emergency_loop: u64,
}

fn default_60() -> u64 {
    60
}
fn default_20() -> u64 {
    20
}
fn default_3() -> u64 {
    3
}
