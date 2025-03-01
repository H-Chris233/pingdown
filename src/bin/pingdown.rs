#![allow(dead_code)]
#![allow(unused)]

mod libs;

use crate::libs::regex::{check_cli};
use crate::libs::loops::normal_loop;
use crate::libs::info::*;
use crate::libs::io::*;
use pingdown::{Cli, Info};
use clap::Parser;

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
    output_info(&info);
    #[cfg(windows)]
    cmd_to_utf8();
    
    normal_loop(&info.vec_address, &info);
}



/// Displays configuration details and initialization status
fn output_struct_info<S: StructInfo>(info: &S) {
    println!("{:#?}", info);
    println!("Initializing monitoring process...");
}








