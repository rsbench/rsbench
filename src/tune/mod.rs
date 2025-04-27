use crate::tune::speedtest::run_speedtest_single_multi;
use crate::GLOBAL_STRING;
use crate::{config, global_println};
use std::fmt::Write;

mod speedtest;

pub fn run_tune(args: &config::Config) {
    if args.speedtest {
        if args.speedtest && args.custom_speedtest_host.is_none() {
            run_speedtest(None);
        }
        if args.custom_speedtest_host.is_some() {
            run_speedtest(args.custom_speedtest_host.as_deref());
        }
    } else {
        run_speedtest(args.custom_speedtest_host.as_deref());
    }
}

fn run_speedtest(custom_host: Option<&str>) {
    if let Some(host) = custom_host {
        std::env::set_var("CUSTOM_SPEEDTEST_SERVER", host);
    }
    run_speedtest_single_multi();
    println!();
    global_println!();
}
