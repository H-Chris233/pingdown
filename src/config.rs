use colored::Colorize;
use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::cli::Cli;

pub const ENV_CONFIG_PATH: &str = "PINGDOWN_CONFIG";
pub const DEFAULT_CONFIG_PATH: &str = "./config.json";
pub const DEFAULT_NORMAL_SECS: u64 = 60;
pub const DEFAULT_EMERGENCY_SECS: u64 = 20;
pub const DEFAULT_EMERGENCY_RETRIES: u32 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorConfig {
    pub targets: Vec<String>,
    pub strict: bool,
    pub normal_interval: Duration,
    pub emergency_interval: Duration,
    pub emergency_retries: NonZeroU32,
    pub quiet: bool,
    pub status_only: bool,
    pub progress: bool,
    pub verbose: u8,
}

impl MonitorConfig {
    pub fn normal_interval_secs(&self) -> u64 {
        self.normal_interval.as_secs()
    }

    pub fn emergency_interval_secs(&self) -> u64 {
        self.emergency_interval.as_secs()
    }

    pub fn emergency_retry_attempts(&self) -> u32 {
        self.emergency_retries.get()
    }
}

pub trait ConfigLoader {
    fn load(&self, path: &Path) -> Result<FileConfig, ConfigError>;
}

#[derive(Default)]
pub struct JsonConfigLoader;

impl ConfigLoader for JsonConfigLoader {
    fn load(&self, path: &Path) -> Result<FileConfig, ConfigError> {
        let contents = fs::read_to_string(path).map_err(|source| ConfigError::io(path, source))?;
        serde_json::from_str(&contents).map_err(|err| ConfigError::parse(path, err))
    }
}

pub fn build_monitor_config(cli: &Cli) -> Result<MonitorConfig, ConfigError> {
    build_monitor_config_with_loader(cli, &JsonConfigLoader::default())
}

pub fn build_monitor_config_with_loader(
    cli: &Cli,
    loader: &dyn ConfigLoader,
) -> Result<MonitorConfig, ConfigError> {
    let mut builder = MonitorConfigBuilder::default();

    if let Some(path) = resolve_config_path(cli) {
        let config = loader.load(&path)?;
        builder.merge_file(config, &path);
    }

    builder.merge_cli(cli);
    builder.build()
}

fn resolve_config_path(cli: &Cli) -> Option<PathBuf> {
    if let Some(path) = &cli.config {
        return Some(path.clone());
    }

    if cli.read_json {
        return Some(PathBuf::from(DEFAULT_CONFIG_PATH));
    }

    if let Ok(value) = env::var(ENV_CONFIG_PATH) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }

    None
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct FileConfig {
    #[serde(alias = "address", alias = "addresses")]
    targets: Option<Vec<String>>,
    strict: Option<bool>,
    #[serde(alias = "secs-for-normal-loop")]
    normal_secs: Option<u64>,
    #[serde(alias = "secs-for-emergency-loop")]
    emergency_secs: Option<u64>,
    #[serde(alias = "times-for-emergency-loop")]
    emergency_retries: Option<u32>,
    quiet: Option<bool>,
    #[serde(alias = "status_only", alias = "status-only")]
    status_only: Option<bool>,
    progress: Option<bool>,
    verbose: Option<u8>,
}

#[derive(Debug, Default)]
struct MonitorConfigBuilder {
    targets: Option<FieldValue<Vec<String>>>,
    strict: Option<FieldValue<bool>>,
    normal_secs: Option<FieldValue<u64>>,
    emergency_secs: Option<FieldValue<u64>>,
    emergency_retries: Option<FieldValue<u32>>,
    quiet: Option<FieldValue<bool>>,
    status_only: Option<FieldValue<bool>>,
    progress: Option<FieldValue<bool>>,
    verbose: Option<FieldValue<u8>>,
}

impl MonitorConfigBuilder {
    fn merge_file(&mut self, cfg: FileConfig, path: &Path) {
        let prefix = |key: &str| format!("{}:{}", path.display(), key);

        if let Some(targets) = cfg.targets {
            self.targets = Some(FieldValue::new(targets, prefix("address")));
        }
        if let Some(strict) = cfg.strict {
            self.strict = Some(FieldValue::new(strict, prefix("strict")));
        }
        if let Some(value) = cfg.normal_secs {
            self.normal_secs = Some(FieldValue::new(value, prefix("secs-for-normal-loop")));
        }
        if let Some(value) = cfg.emergency_secs {
            self.emergency_secs = Some(FieldValue::new(value, prefix("secs-for-emergency-loop")));
        }
        if let Some(value) = cfg.emergency_retries {
            self.emergency_retries = Some(FieldValue::new(value, prefix("times-for-emergency-loop")));
        }
        if let Some(value) = cfg.quiet {
            self.quiet = Some(FieldValue::new(value, prefix("quiet")));
        }
        if let Some(value) = cfg.status_only {
            self.status_only = Some(FieldValue::new(value, prefix("status-only")));
        }
        if let Some(value) = cfg.progress {
            self.progress = Some(FieldValue::new(value, prefix("progress")));
        }
        if let Some(value) = cfg.verbose {
            self.verbose = Some(FieldValue::new(value, prefix("verbose")));
        }
    }

    fn merge_cli(&mut self, cli: &Cli) {
        if !cli.targets.is_empty() {
            self.targets = Some(FieldValue::new(cli.targets.clone(), "cli.targets".to_string()));
        }
        if cli.strict {
            self.strict = Some(FieldValue::new(true, "cli --strict/-s".to_string()));
        }
        if let Some(value) = cli.normal_interval {
            self.normal_secs = Some(FieldValue::new(value, "cli --normal/-n".to_string()));
        }
        if let Some(value) = cli.emergency_interval {
            self.emergency_secs = Some(FieldValue::new(value, "cli --emergency/-e".to_string()));
        }
        if let Some(value) = cli.emergency_retries {
            self.emergency_retries = Some(FieldValue::new(value, "cli --tries/-t".to_string()));
        }
        if cli.quiet {
            self.quiet = Some(FieldValue::new(true, "cli --quiet/-q".to_string()));
        }
        if cli.status_only {
            self.status_only = Some(FieldValue::new(true, "cli --status-only".to_string()));
        }
        if cli.progress {
            self.progress = Some(FieldValue::new(true, "cli --progress".to_string()));
        }
        if cli.verbose > 0 {
            self.verbose = Some(FieldValue::new(cli.verbose, "cli --verbose/-v".to_string()));
        }
    }

    fn build(self) -> Result<MonitorConfig, ConfigError> {
        let FieldValue { value: targets, path: targets_path } = self.targets.ok_or_else(|| {
            ConfigError::validation(
                "cli.targets|config.address",
                "at least one target must be provided via CLI or configuration file",
            )
        })?;

        if targets.is_empty() {
            return Err(ConfigError::validation(
                targets_path,
                "the targets list cannot be empty",
            ));
        }

        let re_address = Regex::new(r"^(?:(?:https?|ftp|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$")
            .expect("valid address regex");

        for (idx, target) in targets.iter().enumerate() {
            if !re_address.is_match(target) {
                return Err(ConfigError::validation(
                    format!("{}[{}]", targets_path, idx),
                    format!("'{}' is not a valid host, IP, or URL", target),
                ));
            }
        }

        let (normal_secs, normal_path) = match self.normal_secs {
            Some(FieldValue { value, path }) => (value, Some(path)),
            None => (DEFAULT_NORMAL_SECS, None),
        };

        if normal_secs == 0 {
            return Err(ConfigError::validation(
                normal_path.unwrap_or_else(|| "defaults.normal_interval".to_string()),
                "normal interval must be greater than zero seconds",
            ));
        }

        let (emergency_secs, emergency_path) = match self.emergency_secs {
            Some(FieldValue { value, path }) => (value, Some(path)),
            None => (DEFAULT_EMERGENCY_SECS, None),
        };

        if emergency_secs == 0 {
            return Err(ConfigError::validation(
                emergency_path.unwrap_or_else(|| "defaults.emergency_interval".to_string()),
                "emergency interval must be greater than zero seconds",
            ));
        }

        let (retries_value, retries_path) = match self.emergency_retries {
            Some(FieldValue { value, path }) => (value, Some(path)),
            None => (DEFAULT_EMERGENCY_RETRIES, None),
        };

        let emergency_retries = NonZeroU32::new(retries_value).ok_or_else(|| {
            ConfigError::validation(
                retries_path.unwrap_or_else(|| "defaults.emergency_retries".to_string()),
                "emergency retry attempts must be at least 1",
            )
        })?;

        let strict = self.strict.map(|FieldValue { value, .. }| value).unwrap_or(false);
        let quiet = self.quiet.map(|FieldValue { value, .. }| value).unwrap_or(false);
        let status_only = self.status_only.map(|FieldValue { value, .. }| value).unwrap_or(false);
        let progress = self.progress.map(|FieldValue { value, .. }| value).unwrap_or(false);
        let verbose = self.verbose.map(|FieldValue { value, .. }| value).unwrap_or(0);

        Ok(MonitorConfig {
            targets,
            strict,
            normal_interval: Duration::from_secs(normal_secs),
            emergency_interval: Duration::from_secs(emergency_secs),
            emergency_retries,
            quiet,
            status_only,
            progress,
            verbose,
        })
    }
}

#[derive(Debug, Clone)]
struct FieldValue<T> {
    value: T,
    path: String,
}

impl<T> FieldValue<T> {
    fn new(value: T, path: String) -> Self {
        Self { value, path }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io { path: PathBuf, source: io::Error },
    Parse { path: PathBuf, message: String },
    Validation { field_path: String, message: String },
}

impl ConfigError {
    fn io(path: &Path, source: io::Error) -> Self {
        Self::Io { path: path.to_path_buf(), source }
    }

    fn parse(path: &Path, err: serde_json::Error) -> Self {
        Self::Parse { path: path.to_path_buf(), message: err.to_string() }
    }

    fn validation(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation { field_path: path.into(), message: message.into() }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io { path, source } => write!(f, "failed to read configuration at '{}': {}", path.display(), source),
            ConfigError::Parse { path, message } => write!(f, "failed to parse configuration at '{}': {}", path.display(), message),
            ConfigError::Validation { field_path, message } => write!(f, "invalid configuration for '{}': {}", field_path, message),
        }
    }
}

impl std::error::Error for ConfigError {}

pub trait OutputInfo: Debug {
    fn output_info(&self) {
        println!("{:#?}", self);
        println!("Initializing monitoring process...");
    }
}

impl OutputInfo for MonitorConfig {
    fn output_info(&self) {
        println!("{} Effective configuration", "[CONFIG]".bold());
        println!("  targets     : {}", self.targets.join(", "));
        println!("  strict      : {}", self.strict);
        println!("  normal      : {}s", self.normal_interval_secs());
        println!(
            "  emergency   : {}s x{}",
            self.emergency_interval_secs(),
            self.emergency_retry_attempts()
        );
        println!("  verbose     : {}", self.verbose);
        println!("  quiet       : {}", self.quiet);
        println!("  status-only : {}", self.status_only);
        println!("  progress    : {}", self.progress);
        println!("{} Initializing monitoring process...", "[INIT]".bold());
    }
}
