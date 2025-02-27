pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "pingdown")]
#[command(author = "H-Chris233")]
#[command(version = "1.1.1")]
pub struct Cli {
    /// Target IP address(es) or domain name(s) to check
    pub vec_ip: Vec<String>,
    /// Enable strict verification mode
    /// 
    /// In strict mode, all targets must be reachable for the check to be considered successful.
    /// By default, the check succeeds if any target is reachable.
    #[arg(short, long)]
    pub strict: bool,
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
