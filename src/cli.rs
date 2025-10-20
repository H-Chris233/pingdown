use clap::{ArgAction, Parser, ValueHint};
use regex::Regex;
use crate::system::error;

#[derive(Parser, Debug)]
#[command(
    name = "pingdown",
    author = "H-Chris233",
    version = "1.4.8",
    about = "A tiny network connectivity monitor that can gracefully shut down the system when connectivity is lost.",
    long_about = "Pingdown continuously pings one or more targets and reports their status.\n\
    It supports 'any-success' (default) and 'strict all-success' modes, configurable \n\
    normal/emergency intervals, optional progress indicator, and rich CLI output.",
    after_help = "EXAMPLES:\n  # Check one target every 60s (default)\n  pingdown 8.8.8.8\n\n  # Strict mode, two targets, normal=30s, emergency=10s, attempts=5\n  pingdown -s -n 30 -e 10 -t 5 1.1.1.1 8.8.8.8\n\n  # Use configuration file\n  pingdown --config ./config.json\n\n  # Quiet summary-only output, show progress spinner\n  pingdown --status-only --progress 1.1.1.1\n"
)]
pub struct Cli {
    /// Target IP address(es) or domain name(s) to check
    #[arg(value_name = "TARGETS", value_hint = ValueHint::Hostname)]
    pub vec_address: Vec<String>,

    /// Enable strict verification mode (all targets must succeed).
    /// By default, the check passes if any target succeeds.
    #[arg(short, long)]
    pub strict: bool,

    /// Read configuration from a JSON file (same format as README). When provided,
    /// CLI addresses and numeric options are ignored and values from the file are used.
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<String>,

    /// Deprecated: read ./config.json from current directory. Use --config instead.
    #[arg(short, long, hide = false)]
    pub read_json: bool,

    /// Interval (in seconds) between regular checks
    #[arg(short = 'n', long = "normal", default_value = "60", value_name = "SECS")]
    pub secs_for_normal_loop: String,

    /// Interval (in seconds) between emergency retries
    #[arg(short = 'e', long = "emergency", default_value = "20", value_name = "SECS")]
    pub secs_for_emergency_loop: String,

    /// Maximum number of emergency retry attempts before shutdown
    #[arg(short = 't', long = "tries", default_value = "3", value_name = "NUM")]
    pub times_for_emergency_loop: String,

    /// Increase output verbosity (-v, -vv). Conflicts with --quiet and --status-only
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count, conflicts_with_all = ["quiet", "status_only"])]
    pub verbose: u8,

    /// Suppress per-target messages; only summaries are printed. Conflicts with --verbose
    #[arg(short = 'q', long = "quiet", conflicts_with = "verbose")]
    pub quiet: bool,

    /// Only print structured status summaries (no per-target ping logs)
    #[arg(long = "status-only")]
    pub status_only: bool,

    /// Show a progress spinner while waiting between checks
    #[arg(long = "progress", default_value_t = false)]
    pub progress: bool,
}

/// Validates CLI arguments:
/// - Requires at least one address unless using --config/-r
/// - Checks IP/URL format compliance
/// - Terminates on errors with alerts
pub fn validate_cli(cli: &Cli) {
    let re_address = match Regex::new(r"^(?:(?:https?|ftp|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$") {
        Ok(re_ip) => re_ip,
        Err(err) => error(&format!("Regex compile failed: {} [validate_cli]", err)),
    };

    // If a config file is provided or legacy -r is used, skip positional validation
    if cli.config.is_none() && !cli.read_json {
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
}
