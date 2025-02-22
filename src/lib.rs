pub use clap::Parser;
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "ping_shutdown")]
#[command(author = "H-Chris233")]
#[command(version = "0.0.2")]
pub struct ArgsIn{
    ///the ip address or website you want to check
    //#[arg(short, long, default_values_t = vec!["bing.com".to_string(), "apple.com".to_string()])]
    #[arg(short, long, default_value = "bing.com")]
    pub ip: String,
    ///use -o to active when any connection losts
    #[arg(short, long, default_value = "None")]
    pub or: String,
    ///time between two normal check
    #[arg(short = 'n', long, default_value = "60")]
    pub secs_for_normal_loop: String,
    ///time between two emegency check
    #[arg(short = 'e', long, default_value = "20")]
    pub secs_for_emergency_loop: String,
    ///times for emergency lopp
    #[arg(short, long, default_value = "3")]
    pub times_for_emergency_loop: String,
}












