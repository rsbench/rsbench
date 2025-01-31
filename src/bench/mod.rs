use crate::GLOBAL_STRING;
use crate::{config, global_println};
use paris::{error, warn};
use std::fmt::Write;

mod fibonacci;
mod mem;
pub(crate) mod network;

pub fn run_bench(args: &config::Config) {
    if args.network || args.fib || args.disk || args.mem {
        println!("BENCH: ");
        global_println!("BENCH: ");
        if args.fib {
            run_fib();
        }
        if args.disk {
            run_disk();
        }
        if args.mem {
            run_mem();
        }
        if args.network {
            run_network();
        }
    } else {
        run_fib();
        run_disk();
        run_mem();
        run_network();
    }
    println!();
    global_println!();
}

fn run_network() {
    network::ping();
    network::start_speedtest();
    network::start_multithread_speedtest(4);
}

fn run_fib() {
    if cfg!(debug_assertions) {
        warn!("This program should be built in release mode for accurate benchmarking");
        global_println!("⚠ This program should be built in release mode for accurate benchmarking");
        warn!("Skipping fibonacci benchmark");
        global_println!("⚠ Skipping fibonacci benchmark");
        println!();
    } else {
        let total_threads = u32::try_from(sysinfo::System::new_all().cpus().len()).unwrap_or(1);
        fibonacci::run_fibonacci();
        fibonacci::run_fibonacci_mt(total_threads);
    }
}

fn run_disk() {
    error!("Disk test feature is not yet completed.");
    global_println!("✖ Disk test feature is not yet completed.");
}

fn run_mem() {
    mem::run_men_test();
}
