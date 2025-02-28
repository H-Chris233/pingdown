#![allow(dead_code)]
#![allow(unused)]

mod libs;

use pingdown::*;
use crate::libs::regex::check_cli;
use crate::libs::loops::normal_loop;
use crate::libs::io::*;

/// Handles command-line argument processing and terminal encoding configuration.
/// Serves as the main entry point for the application.
fn main() {
    let cli = Cli::parse();
    check_cli(&cli);
    output_message(&cli);
    #[cfg(windows)]
    cmd_to_utf8();
    
    normal_loop(&cli.vec_ip, &cli);
}

/// Displays configuration details and initialization status
fn output_message(cli: &Cli) {
    println!("{:#?}", cli);
    println!("Initializing monitoring process...");
}








