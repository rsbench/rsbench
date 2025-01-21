use crate::tune::ip_check::ip_all;
use crate::tune::speedtest::run_speedtest_single_multi;
use crate::GLOBAL_STRING;
use crate::{config, global_println};
use futures::executor::block_on;
use std::fmt::Write;

mod ip_check;
mod speedtest;

pub fn run_tune(args: &config::Config) {
    if args.ip || args.speedtest {
        if args.ip {
            check_ip_status();
        }
        if args.speedtest && args.custom_speedtest_host.is_none() {
            run_speedtest(None);
        }
        if args.custom_speedtest_host.is_some() {
            run_speedtest(args.custom_speedtest_host.as_deref());
        }
    } else {
        check_ip_status();
        run_speedtest(args.custom_speedtest_host.as_deref());
    }
}

fn check_ip_status() {
    block_on(ip_all());
    println!();
    global_println!();
}

fn run_speedtest(custom_host: Option<&str>) {
    if let Some(host) = custom_host {
        std::env::set_var("CUSTOM_SPEEDTEST_SERVER", host);
    }
    run_speedtest_single_multi();
    println!();
    global_println!();
}
