use clap::{ArgAction, Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
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
    pub targets: Vec<String>,

    /// Enable strict verification mode (all targets must succeed).
    /// By default, the check passes if any target succeeds.
    #[arg(short, long)]
    pub strict: bool,

    /// Read configuration from a JSON file (same format as README).
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Deprecated: read ./config.json from current directory. Use --config instead.
    #[arg(short = 'r', long = "read-json", hide = false)]
    pub read_json: bool,

    /// Interval (in seconds) between regular checks (default: 60)
    #[arg(short = 'n', long = "normal", value_name = "SECS")]
    pub normal_interval: Option<u64>,

    /// Interval (in seconds) between emergency retries (default: 20)
    #[arg(short = 'e', long = "emergency", value_name = "SECS")]
    pub emergency_interval: Option<u64>,

    /// Maximum number of emergency retry attempts before shutdown (default: 3)
    #[arg(short = 't', long = "tries", value_name = "NUM")]
    pub emergency_retries: Option<u32>,

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
