use regex::Regex;
use crate::libs::io::error;
use pingdown::Cli;

/// Validates CLI arguments:
/// - Requires at least one address
/// - Checks IP/URL format compliance
/// - Terminates on errors with alerts
pub fn check_cli(cli: &Cli) {
    // Address format regex supports:
    // - IPv4/IPv6
    // - Domains
    // - HTTP/HTTPS/FTP URLs
    // - Ports and paths
    let re_address = match Regex::new(r"^(?:(?:https?|ftp|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$") {
        Ok(re_ip) => re_ip,
        Err(err) => error(&format!("Regex compile failed: {} [check_cli]", err)),
    };

    // Require minimum one address
    if cli.vec_address.is_empty() {
        println!("Provide at least one IP/website\nUse -h for help");
        error("No target addresses detected");
    }

    // Validate each address format
    for ip in &cli.vec_address {
        if !re_address.is_match(ip) {
            error(&format!("Invalid address [check_cli]\n{}: Verify format", ip));
        }
    }
}
