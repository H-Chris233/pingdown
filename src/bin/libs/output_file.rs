use pingdown::RuntimeInfo;
use std::fs::File;

pub fn write_info(runtime_info: &RuntimeInfo) -> Result<(), std::io::Error> {
    let output_str = File::create("pingdown_runtime_info.txt")?;
    
    
    
    
    Ok(())
}













