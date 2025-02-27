pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[command(name = "pingdown")]
#[command(author = "H-Chris233")]
#[command(version = "0.2.4")]
pub struct Cli{
    ///the ip address or website you want to check
    pub vec_ip: Vec<String>,
    ///Active strict mode.
    /// Strict mode requires all targets to be reachable
    /// Default mode accepts any successful connection
    #[arg(short, long)]
    pub strict: bool,
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












