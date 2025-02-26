pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "ping_shutdown")]
#[command(author = "H-Chris233")]
#[command(version = "0.2.1")]
pub struct Cli{
    ///the ip address or website you want to check
    #[arg(short, default_value = "bing.com", value_name = "IP WEBSITE ...")]
    pub vec_ip: Vec<String>,
    ///use -a to active shutdown when any connection losts
    #[arg(short)]
    pub and_or: bool,
    ///time between two normal check
    #[arg(short = 'n', default_value = "60")]
    pub secs_for_normal_loop: String,
    ///time between two emegency check
    #[arg(short = 'e', default_value = "20")]
    pub secs_for_emergency_loop: String,
    ///times for emergency lopp
    #[arg(short, default_value = "3")]
    pub times_for_emergency_loop: String,
}












