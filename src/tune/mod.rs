use crate::config;
use crate::tune::ip_check::ip_all;
use futures::executor::block_on;

mod ip_check;

pub fn run_tune(args: &config::Config) {
    if args.ip {
        if args.ip {
            check_ip_status();
        }
    } else {
        check_ip_status();
    }
}

fn check_ip_status() {
    block_on(ip_all());
}
