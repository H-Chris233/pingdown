use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use clap::Parser;

use crate::cli::{Cli, validate_cli};
use crate::config::{Config, read_json, read_json_with_path, from_cli, OutputInfo};
use crate::monitor::normal_loop;
use crate::runtime::Metrics;
use crate::signals::install_ctrlc_handler;
use crate::system::{DefaultSystem, System};

/// High-level application orchestrator. Wires together CLI parsing, configuration
/// resolution, monitoring loop scheduling, and graceful shutdown handling.
pub struct App<S: System + Send + Sync + 'static = DefaultSystem> {
    system: S,
}

impl Default for App<DefaultSystem> {
    fn default() -> Self {
        Self { system: DefaultSystem::new() }
    }
}

impl<S: System + Send + Sync + 'static> App<S> {
    pub fn new(system: S) -> Self { Self { system } }

    /// Run the application. This function blocks indefinitely running the monitoring loop
    /// until a shutdown signal is received (Ctrl-C), upon which final metrics are flushed
    /// and the process exits.
    pub fn run(self) {
        // 1) Parse CLI
        let cli = Cli::parse();

        // 2) Prepare runtime state and graceful-shutdown signal wiring
        let metrics = Arc::new(Mutex::new(Metrics::new()));
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        install_ctrlc_handler(shutdown_flag.clone(), metrics.clone());

        // 3) Resolve configuration from JSON or CLI
        let config: Config = if let Some(path) = &cli.config {
            read_json_with_path(Some(path))
        } else if cli.read_json {
            read_json()
        } else {
            validate_cli(&cli);
            from_cli(cli)
        };

        // 4) Perform platform-specific console tweaks (no-op on Unix)
        self.system.console_setup();

        // 5) Output effective configuration and start normal monitoring loop
        config.output_info();
        normal_loop(config, metrics, &self.system);
    }
}
