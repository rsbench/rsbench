use crate::bench::disk::run_disk_speed_test;
use crate::config;
use paris::{info, warn};

mod disk;
mod fibonacci;
mod mem;
pub(crate) mod network;

pub fn run_bench(args: &config::Config) {
    if args.network || args.fib || args.disk || args.mem {
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
        if args.mem {
            run_mem();
        } else {
            info!("Memory benchmark is disabled");
        }
        if args.network {
            run_network();
        } else {
            info!("Network benchmark is disabled");
        }
    } else {
        run_fib();
        run_disk();
        run_mem();
        run_network();
    }
}

fn run_network() {
    network::ping();
    network::start_speedtest();
    network::start_multithread_speedtest(4);
}

fn run_fib() {
    if cfg!(debug_assertions) {
        warn!("This program should be built in release mode for accurate benchmarking");
        warn!("Skipping fibonacci benchmark");
        println!();
    } else {
        let total_threads = u32::try_from(sysinfo::System::new_all().cpus().len()).unwrap_or(1);
        fibonacci::run_fibonacci();
        fibonacci::run_fibonacci_mt(total_threads);
    }
}

fn run_disk() {
    run_disk_speed_test();
}

fn run_mem() {
    mem::run_men_test();
}
