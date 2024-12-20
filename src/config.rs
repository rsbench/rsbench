use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rsbench", version, about)]
pub struct Config {
    /// Run information gathering
    #[arg(short, long)]
    pub info: bool,

    /// Run the benchmark
    #[arg(short, long)]
    pub bench: bool,

    /// Run Media Unlock test
    #[arg(short, long)]
    pub unlock: bool,

    /// Run System Tuning
    #[arg(short, long)]
    pub tune: bool,
}
