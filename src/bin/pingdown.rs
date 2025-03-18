#![allow(dead_code)]
#![allow(unused)]

mod libs;

use crate::libs::check_input:: {
    check_cli
};
use crate::libs::loops::normal_loop;
use crate::libs::struct_info::*;
use crate::libs::output_file::*;
use crate::libs::io::error;
use pingdown::Cli;
use clap::Parser;

use ctrlc;
use std::sync::atomic:: {AtomicBool, Ordering};
use std::sync:: {Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam_channel:: {select, tick};


fn main() {
    let runtime_info = Arc::new(Mutex::new(RuntimeInfo::new()));
    let runtime_info_clone = Arc::clone(&runtime_info);
    // 共享原子标志位和信号通道
    let ctrlc_flag = Arc::new(AtomicBool::new(false));
    let ctrlc_clone = ctrlc_flag.clone();
    let ticker = tick(Duration::from_millis(80));

    // 注册 Ctrl-C 处理器（即时触发）
    match ctrlc::set_handler(move || {
        ctrlc_clone.store(true, Ordering::SeqCst);
    }) {
        Ok(()) => {},
        Err(err) => error(&format!("setting ctrl+c handler[{}]", err)),
    }

    thread::spawn(move || {
        loop {
            select! {
                recv(ticker) -> _ => {
                    if ctrlc_flag.load(Ordering::SeqCst) {
                        println!("Write file and exit...");
                        let output = match runtime_info_clone.lock() {
                            Ok(output) => output,
                            Err(err) => error(&format!("locking files.[{}]", err))
                        };
                        output.write();
                        std::process::exit(0);
                    }
                }
            }
        }
    });
    entry(runtime_info);
}

/// Handles command-line argument processing and terminal encoding configuration.
/// Serves as the main entry point for the application.
fn entry(runtime_info: Arc<Mutex<RuntimeInfo>>) {
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
    normal_loop(&info, runtime_info);
}







