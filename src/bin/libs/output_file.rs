use std::fs;
use crate::libs::io::error;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct RuntimeInfo {
    #[serde(alias = "total-succeeds")]
    pub total_succeeds: u64,
    #[serde(alias = "total-failures")]
    pub total_failures: u64,
    #[serde(alias = "total-normal-loop-times")]
    pub total_normal_loop_times: u64,
    #[serde(alias = "total-emergency-loop-times")]
    pub total_emergency_loop_times: u64,
}


impl RuntimeInfo {
    pub fn new() -> RuntimeInfo {
        let output = RuntimeInfo {
            total_succeeds: 0,
            total_failures: 0,
            total_normal_loop_times: 0,
            total_emergency_loop_times: 0,
        };
        output
    }
    pub fn output(&self) {
        println!("{:#?}", self);
    }
    pub fn write(&self) {
        match fs::write("pingdown_runtime_info.txt", &format!("{:#?}", self)) {
            Ok(()) => {},
            Err(_) => error("writing output file, please check your permission"),
        }
    }
}









