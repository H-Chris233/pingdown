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
    
}
