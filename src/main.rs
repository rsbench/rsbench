// #![warn(clippy::all, clippy::pedantic)]

use crate::utils::{clear_screen, set_default_colour, set_random_colour};
use clap::Parser;
use paris::{info, warn};

mod bench;
mod config;
mod info;
mod tune;
mod unlock_test;
mod utils;

#[tokio::main]
async fn main() {
    let args = config::Config::parse();

    if args.no_color {
        std::env::set_var("RSBENCH_NO_COLOR", "1");
    }

    if !args.no_cls {
        clear_screen();
    }

    info!("RSBench v{}", env!("CARGO_PKG_VERSION"));
    warn!("This is Alpha software. Things may and will break.");
    if !args.no_logo {
        print_ascii_art();
    }

    if args.info {
        info::run_info();
    }
    if args.bench {
        bench::run_bench(&args);
    }
    if args.unlock {
        unlock_test::run_unlock_test(&args);
    }
    if args.tune {
        tune::run_tune();
    }
    if !args.info && !args.bench && !args.unlock && !args.tune {
        warn!("No parameter is currently specified, info mode will be used by default.");
        info::run_info();
    }
}

fn print_ascii_art() {
    let ascii_art = r"  _____   _____ ____                  _
 |  __ \ / ____|  _ \                | |
 | |__) | (___ | |_) | ___ _ __   ___| |__
 |  _  / \___ \|  _ < / _ \ '_ \ / __| '_ \
 | | \ \ ____) | |_) |  __/ | | | (__| | | |
 |_|  \_\_____/|____/ \___|_| |_|\___|_| |_|

";
    for ascii in ascii_art.chars() {
        set_random_colour();
        print!("{ascii}");
    }
    set_default_colour();
}
