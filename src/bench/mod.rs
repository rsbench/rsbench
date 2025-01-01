use crate::bench::disk::run_disk_speed_test;
use crate::config;
use paris::{info, warn};

mod disk;
mod fibonacci;
mod network;

pub fn run_bench(args: &config::Config) {
    if args.network || args.fib || args.disk {
        if args.network {
            run_network();
        } else {
            info!("Network benchmark is disabled");
        }
        if args.fib {
            run_fib();
        } else {
            info!("FIB benchmark is disabled");
        }
        if args.disk {
            run_disk();
        } else {
            info!("Disk benchmark is disabled");
        }
    } else {
        run_network();
        run_fib();
        run_disk();
    }
}

fn run_network() {
    network::ping();
    network::start_speedtest();
    network::start_multithread_speedtest(4);
}

fn run_fib() {
    if !cfg!(debug_assertions) {
        let total_threads = sysinfo::System::new_all().cpus().len() as u32;
        fibonacci::run_fibonacci();
        fibonacci::run_fibonacci_mt(total_threads);
    } else {
        warn!("This program should be built in release mode for accurate benchmarking");
        warn!("Skipping fibonacci benchmark");
        println!();
    }
}

fn run_disk() {
    run_disk_speed_test();
}
