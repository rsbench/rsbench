use paris::warn;

mod fibonacci;
mod network;
pub fn run_bench() {
    network::ping();
    network::start_speedtest();
    network::start_multithread_speedtest(4);
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
