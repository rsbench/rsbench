use clap::Parser;
use paris::{info, warn};
mod bench;
mod config;
mod info;
mod media;
mod tune;

#[tokio::main]
async fn main() {
    info!("RSBench v{}", env!("CARGO_PKG_VERSION"));
    // Exit if not built as release
    if cfg!(debug_assertions) {
        warn!("This program should be built in release mode for accurate benchmarking");
        warn!("Results are very likely to be inaccurate");
    }
    let args = config::Config::parse();
    println!("{:?}", args);
    if args.info {
        info::run_info();
    }
    if args.bench {
        bench::run_bench();
    }
    if args.media {
        media::run_media();
    }
    if args.tune {
        tune::run_tune();
    }
}
