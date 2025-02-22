use clap::Parser;
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "ping_shutdown")]
#[command(author = "H-Chris233")]
#[command(version = "0.0.2")]
pub struct Args{
    #[arg(short, long, default_values_t = vec!["bing.com".to_string(), "apple.com".to_string()])]
    pub ip: Vec<String>,
    #[arg(short, long, default_value = None)]
    pub or: Option<String>,
    #[arg(short, long, default_value = None)]
    pub secs_for_loop: Option<String>,
    #[arg(short, long, default_value = None)]
    pub times_for_emergency_loop: Option<String>,
}

