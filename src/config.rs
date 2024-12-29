use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "RSBench", version, about)]
pub struct Config {
    /// Run information gathering
    #[arg(short, long)]
    pub info: bool,

    /// Run the benchmark
    #[arg(short, long)]
    pub bench: bool,

    /// Run services unlock test
    #[arg(short, long)]
    pub unlock: bool,

    // Set services unlock region
    #[arg(long)]
    pub region: Option<Vec<UnlockRegion>>,

    /// Run system tuning
    #[arg(short, long)]
    pub tune: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum UnlockRegion {
    HK,
    MO,
    TW,
    JP,
    CN,
    Asia,
    Euro,
    Afr,
    UK,
    US,
    Global,
}
