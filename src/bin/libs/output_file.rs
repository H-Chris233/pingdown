use std::fs;
use crate::libs::io::error;
use std::sync:: {
    Arc,
    Mutex
};


#[derive(Debug)]
pub struct RuntimeInfo {
    // #[serde(alias = "total-succeeds")]
    pub total_succeeds: u64,
    // pub total_succeeds: Arc<Mutex<u64>>,
    // #[serde(alias = "total-failures")]
    pub total_failures: u64,
    // pub total_failures: Arc<Mutex<u64>>,
    // #[serde(alias = "total-normal-loop-times")]
    pub total_normal_loop_times: u64,
    // pub total_normal_loop_times: Arc<Mutex<u64>>,
    // #[serde(alias = "total-emergency-loop-times")]
    pub total_emergency_loop_times: u64,
    // pub total_emergency_loop_times: Arc<Mutex<u64>>,
}

pub enum Info {
    Succeeds,
    Failures,
    NormalLoopTimes,
    EmergencyLoopTimes,
}

impl RuntimeInfo {
    pub fn new() -> Self {
        Self {
            total_succeeds: 0,
            // total_succeeds: Arc::new(Mutex::new(0)),
            total_failures: 0,
            // total_failures: Arc::new(Mutex::new(0)),
            total_normal_loop_times: 0,
            // total_normal_loop_times: Arc::new(Mutex::new(0)),
            total_emergency_loop_times: 0,
            // total_emergency_loop_times: Arc::new(Mutex::new(0)),
        }
    }
    pub fn output(&self) {
        println!("{:#?}", self);
    }
    pub fn write(&self) {
        match self {
            RuntimeInfo {
                total_succeeds: 0,
                total_failures: 0,
                total_normal_loop_times: 0,
                total_emergency_loop_times: 0,
            } => {},
            _ => {
                match fs::write("pingdown_runtime_info.txt", &format!("{:#?}", self)) {
                    Ok(_) => {},
                    Err(err) => error(&format!("writing output file[{}], please check your permission.", err)),
                }
            },
        }
    }
}

pub fn add_one(runtime_info: &Arc<Mutex<RuntimeInfo>>, key: Info) {
    let mut runtime_info = match runtime_info.lock() {
        Ok(output) => output,
        Err(err) => error(&format!("locking value [{}]", err)),
    };
    match key {
        Info::Succeeds => {
            runtime_info.total_succeeds += 1;
        }
        Info::Failures => {
            runtime_info.total_failures += 1;
        }
        Info::NormalLoopTimes => {
            runtime_info.total_normal_loop_times += 1;
        }
        Info::EmergencyLoopTimes => {
            runtime_info.total_emergency_loop_times += 1;
        }
    }
}