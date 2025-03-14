use pingdown::RuntimeInfo;
use std::fs::File;

fn write_info(runtime_info: &RuntimeInfo) {
    let output_str = match std::fs::read_to_string("./pingdown_runtime_info.json") {
        Ok(output_str) => output_str,
        Err(_) => {
            File::create("./pingdown_runtime_info.json").expect("...");
            "".to_string()
        }
    };
}













