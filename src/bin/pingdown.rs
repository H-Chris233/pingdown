mod libs;

use crate::libs::regex::{check_cli, cli_to_info};
use crate::libs::loops::normal_loop;
use pingdown::{Cli, Info};
pub use clap::Parser;
use crate::libs::io::*;
use std::fs;

/// Handles command-line argument processing and terminal encoding configuration.
/// Serves as the main entry point for the application.
fn main() {
    let cli = Cli::parse();
    let info = match &cli.read_json {
        true => {
            read_json()
        }
        false => {
            check_cli(&cli);
            cli_to_info(cli)
        }
    };
    output_message(&info);
    #[cfg(windows)]
    cmd_to_utf8();
    
    normal_loop(&info.vec_address, &info);
}

/// Displays configuration details and initialization status
fn output_message(info: &Info) {
    println!("{:#?}", info);
    println!("Initializing monitoring process...");
}

fn read_json() -> Info {
    let json_str = match fs::read_to_string("./config.json") {
        Ok(json_str) => json_str,
        Err(err) => error(&format!("reading JSON file.\n{}", err)),
    };
    let config: Info = match serde_json::from_str(&json_str) {
        Ok(json_info) => json_info,
        Err(err) => error(&format!("parsing JSON file.\n{}", err)),
    };
    config
}
