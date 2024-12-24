use clap::Parser;
use paris::info;
mod bench;
mod config;
mod info;
mod tune;
mod unlock_test;
mod utils;

#[tokio::main]
async fn main() {
    info!("RSBench v{}", env!("CARGO_PKG_VERSION"));

    let args = config::Config::parse();
    // println!("{:?}", args);
    if args.info {
        info::run_info();
    }
    if args.bench {
        bench::run_bench();
    }
    if args.unlock {
        unlock_test::run_unlock_test();
    }
    if args.tune {
        tune::run_tune();
    }
}
