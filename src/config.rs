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

    // Run network speedtest
    #[arg(long, default_value_t = false, requires = "bench")]
    pub network: bool,

    // Run FIB CPU test
    #[arg(long, default_value_t = false, requires = "bench")]
    pub fib: bool,

    /// Run disk speedtest
    #[arg(long, default_value_t = false, requires = "bench")]
    pub disk: bool,

    /// Run services unlock test
    #[arg(short, long)]
    pub unlock: bool,

    // Set services unlock region
    #[arg(long, requires = "unlock")]
    pub region: Option<Vec<UnlockRegion>>,

    /// Run system tuning
    #[arg(short, long)]
    pub tune: bool,

    // Disable color output
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    // Disable clear screen
    #[arg(long, default_value_t = false)]
    pub no_cls: bool,
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
