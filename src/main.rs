// #![warn(clippy::all, clippy::pedantic)]

use crate::utils::{clear_screen, get_usage_count, set_colour, set_default_colour};
use clap::Parser;
use paris::{info, warn};
use termcolor::Color;

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
    if let Ok(usage) = get_usage_count().await {
        set_colour(Color::Magenta);
        println!(
            "This project has been called {} times in total, {} times today",
            usage.1, usage.0
        );
        set_default_colour();
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
        tune::run_tune(&args);
    }
    if !args.info && !args.bench && !args.unlock && !args.tune {
        warn!("No parameter is currently specified, info mode will be used by default.");
        info::run_info();
    }
}

fn print_ascii_art() {
    let ascii_art = r"   ___  _______               __
  / _ \/ __/ _ )___ ___  ____/ /
 / , _/\ \/ _  / -_) _ \/ __/ _ \
/_/|_/___/____/\__/_//_/\__/_//_/
                                 ";
    set_colour(Color::Rgb(255, 182, 193));
    println!("{ascii_art}");
    set_default_colour();
}
