use crate::config;
use crate::tune::ip_check::ip_all;
use futures::executor::block_on;

mod ip_check;
mod speedtest;

pub fn run_tune(args: &config::Config) {
    if args.ip || args.speedtest {
        if args.ip {
            check_ip_status();
        }
        if args.speedtest {
            speedtest();
        }
    } else {
        check_ip_status();
        speedtest();
    }
}

fn check_ip_status() {
    block_on(ip_all());
}

fn speedtest() {
    block_on(speedtest::single_thread_download());
}
