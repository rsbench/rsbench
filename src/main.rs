use clap::Parser;
use paris::{info, warn};
mod bench;
mod config;
mod info;
mod tune;
mod unlock_test;

#[tokio::main]
async fn main() {
    info!("RSBench v{}", env!("CARGO_PKG_VERSION"));
    if cfg!(debug_assertions) {
        warn!("This program should be built in release mode for accurate benchmarking");
        warn!("Results are very likely to be inaccurate");
    }
    let args = config::Config::parse();
    // println!("{:?}", args);
    if args.info {
        info::run_info();
    }
    if args.bench {
        bench::run_bench();
    }
    if args.media {
        unlock_test::run_media();
    }
    if args.tune {
        tune::run_tune();
    }
}
