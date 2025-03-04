#![allow(dead_code)]
#![allow(unused)]

mod libs;

use crate::libs::check_input::{check_cli};
use crate::libs::loops::normal_loop;
use crate::libs::struct_info::*;
use crate::libs::io::*;
use pingdown::{Cli, RuntimeInfo};
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
    #[cfg(windows)]
    cmd_to_utf8();
    
    info.output_info();
    let mut runtime_info = RuntimeInfo::new();
    normal_loop(&info, &mut runtime_info);
}









