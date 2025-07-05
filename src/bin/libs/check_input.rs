use crate::Cli;
use regex::Regex;
use anyhow::{Context, Result};
use log::{error, info};

/// Validates CLI arguments:
/// - Requires at least one address
/// - Checks IP/URL format compliance
/// - Returns error with diagnostics
pub fn check_cli(cli: &Cli) -> Result<()> {
    // Address format regex supports:
    // - IPv4/IPv6
    // - Domains
    // - HTTP/HTTPS/FTP URLs
    // - Ports and paths
    let re_address = Regex::new(
        r"^(?:(?:https?|ftp|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$"
    ).context("Regex compile failed in check_cli")?;

    // Require minimum one address
    if cli.vec_address.is_empty() {
        error!("Please provide at least one IP/website, use -h for help");
        return Err(anyhow::anyhow!("No target addresses detected"));
    }

    // Validate each address format
    for ip in &cli.vec_address {
        if !re_address.is_match(ip) {
            error!("Invalid address [check_cli]\n{}: Verify format", ip);
            return Err(anyhow::anyhow!("Invalid address format: {}", ip));
        }
    }

    Ok(())
}
