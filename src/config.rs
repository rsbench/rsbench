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

    #[arg(long, default_value_t = false, requires = "bench")]
    pub mem: bool,

    /// Run services unlock test
    #[arg(short, long)]
    pub unlock: bool,

    // Set services unlock region
    #[arg(long, requires = "unlock")]
    pub region: Option<Vec<UnlockRegion>>,

    /// Run system tuning
    #[arg(short, long)]
    pub tune: bool,

    #[arg(long, requires = "tune")]
    pub ip: bool,

    #[arg(long, requires = "tune")]
    pub speedtest: bool,

    #[arg(long, requires = "speedtest")]
    pub custom_speedtest_host: Option<String>,

    // Disable color output
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    // Disable clear screen
    #[arg(long, default_value_t = false)]
    pub no_cls: bool,

    // Disable logo output
    #[arg(long, default_value_t = false)]
    pub no_logo: bool,

    // Disable usage count output
    #[arg(long, default_value_t = false)]
    pub no_usage: bool,

    // Disable upload test result
    #[arg(long, default_value_t = false)]
    pub no_upload: bool,
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
