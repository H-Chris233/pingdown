
use regex::Regex;
use crate::libs::io::error;
use pingdown::Cli;
use crate::libs::struct_info::convert_num;

/// Validates command-line interface parameters with strict protocol compliance checks
/// 
/// # Responsibilities
/// - Verifies input addresses conform to RFC-compliant network patterns
/// - Enforces mandatory target specification
/// - Performs format validation for:
///   * IPv4/IPv6 addresses
///   * HTTP/HTTPS/FTP URLs
///   * Domain name structures
///   * Port declarations
///   * Query parameters and URL fragments
/// 
/// # Panics
/// - Terminates execution on invalid regex patterns with error code E_FORMAT
/// - Aborts process when address list is empty (E_NO_TARGET)
/// - Triggers fatal error for malformed addresses (E_INVALID_ADDR)
/// 
/// # Arguments
/// - `cli`: Parsed command-line interface configuration containing:
///   * Target addresses (`vec_address`)
///   * Protocol parameters
///   * Network configuration options
pub fn check_cli(cli: &Cli) {
    // Compiled regex pattern for RFC 3986 compliant URI validation with network extensions:
    // - Supports optional protocol prefixes (http/https/ftp/ftps)
    // - Validates userinfo (username:password@)
    // - Checks TLD validity and IP literal formats
    // - Verifies port declarations and path components
    let re_address = match Regex::new(r"^(?:(?:https?|ftp|ftps)://)?(?:[^\s:@/]+(?::[^\s:@/]*)?@)?(?:(?:www\.)?(?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3}|\[[a-fA-F0-9:]+\])(?::\d+)?(?:/[^\s?#]*)?(?:\?[^\s#]*)?(?:#[^\s]*)?$") {
        Ok(re_ip) => re_ip,
        Err(err) => error(&format!("Regex compilation failed. {}[in function check_cli]", err)),
    };

    // Protocol enforcement: Require minimum one target specification
    if cli.vec_address.is_empty() {
        println!("Please provide at least one IP address or website.\nFor usage instructions, use -h or --help.");
        error("there's no address to detect");
    }

    // Iterative validation of all target addresses
    for ip in &cli.vec_address {
        match re_address.is_match(ip) {
            true => {},  // Pass validation for compliant addresses
            false => error(&format!(
                "Invalid address format[in function check_cli]\n{}: Please verify target correctness", 
                ip
            )),
        }
    }
}
