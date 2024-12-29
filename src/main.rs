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
    clear_screen();
    info!("RSBench v{}", env!("CARGO_PKG_VERSION"));
    warn!("This is Alpha software. Thing may and will break.");
    print_ascii_art();

    let args = config::Config::parse();
    if args.info {
        info::run_info();
    }
    if args.bench {
        bench::run_bench();
    }
    if args.unlock {
        unlock_test::run_unlock_test(&args);
    }
    if args.tune {
        tune::run_tune();
    }
}

fn print_ascii_art() {
    let ascii_art = r#"  _____   _____ ____                  _
 |  __ \ / ____|  _ \                | |
 | |__) | (___ | |_) | ___ _ __   ___| |__
 |  _  / \___ \|  _ < / _ \ '_ \ / __| '_ \
 | | \ \ ____) | |_) |  __/ | | | (__| | | |
 |_|  \_\_____/|____/ \___|_| |_|\___|_| |_|

"#;
    for ascii in ascii_art.chars() {
        set_random_colour();
        print!("{}", ascii);
    }
    set_default_colour();
}
