use crate::config;
use crate::tune::ip_check::ip_all;
use crate::tune::speedtest::{get_table, run_multi, run_single};
use crate::utils::clear_last_line;
use futures::executor::block_on;

mod ip_check;
mod speedtest;

pub fn run_tune(args: &config::Config) {
    if args.ip || args.speedtest {
        if args.ip {
            check_ip_status();
        }
        if args.speedtest {
            run_speedtest();
        }
    } else {
        check_ip_status();
        run_speedtest();
    }
}

fn check_ip_status() {
    block_on(ip_all());
}

fn run_speedtest() {
    let (single_download, single_upload) = run_single();
    let (multi_download, multi_upload) = run_multi();

    for _ in 0..((4 + 2 + 1) * 4) {
        clear_last_line();
    }

    let (table_single, table_multi) =
        get_table(single_download, single_upload, multi_download, multi_upload);

    println!("Single Thread Speedtest");
    table_single.printstd();

    println!();
    println!("Multi Thread Speedtest");
    table_multi.printstd();
}
