// #![warn(clippy::all, clippy::pedantic)]

use crate::utils::color::{set_colour, set_default_colour};
use crate::utils::report::get_usage_count;
use crate::utils::report::GLOBAL_STRING;
use crate::utils::term::clear_screen;
use clap::Parser;
use paris::{error, info, warn};
use std::fmt::Write;
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
    global_println!("ℹ RSBench v{}", env!("CARGO_PKG_VERSION"));
    warn!("This is Alpha software. Things may and will break.");
    global_println!("⚠ This is Alpha software. Things may and will break.");

    if !args.no_logo {
        print_ascii_art();
    }
    if let Ok(usage) = get_usage_count().await {
        set_colour(Color::Magenta);
        info!(
            "This project has been called {} times in total, {} times today",
            usage.1, usage.0
        );
        global_println!(
            "This project has been called {} times in total, {} times today",
            usage.1,
            usage.0
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
        global_println!(
            "⚠ No parameter is currently specified, info mode will be used by default."
        );
        info::run_info();
    }

    if !args.no_upload {
        if let Ok(id) = utils::report::post_to_pastebin().await {
            set_colour(Color::Green);
            println!("Result URL: https://rsbench-pastebin.genshinminecraft-d20.workers.dev/{id}");
        }
        match utils::report::post_to_pastebin().await {
            Ok(id) => {
                info!(
                    "Result URL: https://rsbench-pastebin.genshinminecraft-d20.workers.dev/{}",
                    id
                );
            }
            Err(err) => {
                error!("Failed to upload result to pastebin: {}", err);
            }
        }
    }
}

fn print_ascii_art() {
    let ascii_art = r"
██████╗ ███████╗██████╗ ███████╗███╗   ██╗ ██████╗██╗  ██╗
██╔══██╗██╔════╝██╔══██╗██╔════╝████╗  ██║██╔════╝██║  ██║
██████╔╝███████╗██████╔╝█████╗  ██╔██╗ ██║██║     ███████║
██╔══██╗╚════██║██╔══██╗██╔══╝  ██║╚██╗██║██║     ██╔══██║
██║  ██║███████║██████╔╝███████╗██║ ╚████║╚██████╗██║  ██║
╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝╚═╝  ╚═══╝ ╚═════╝╚═╝  ╚═╝
                                                          ";
    set_colour(Color::Rgb(255, 182, 193));
    println!("{ascii_art}");
    global_println!("{}", ascii_art);
    set_default_colour();
}
