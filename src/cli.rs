use clap::Parser;
use regex::Regex;
use crate::system::error;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "pingdown")]
#[command(author = "H-Chris233")]
#[command(version = "1.4.8")]
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

/// Validates CLI arguments:
/// - Requires at least one address
/// - Checks IP/URL format compliance
/// - Terminates on errors with alerts
pub fn validate_cli(cli: &Cli) {
    let re_address = match Regex::new(r"^(?:(?:https?|ftp|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$") {
        Ok(re_ip) => re_ip,
        Err(err) => error(&format!("Regex compile failed: {} [validate_cli]", err)),
    };

    if cli.vec_address.is_empty() {
        println!("\nPlease provide at least one IP/website\nUse -h for help");
        error("No target addresses detected");
    }

    for ip in &cli.vec_address {
        if !re_address.is_match(ip) {
            error(&format!("Invalid address [validate_cli]\n{}: Verify format", ip));
        }
    }
}
