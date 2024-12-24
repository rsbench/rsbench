use paris::warn;

mod fibonacci;
mod network;
pub fn run_bench() {
    network::ping();
    network::start_speedtest();
    network::start_multithread_speedtest(4);
    if !cfg!(debug_assertions) {
        fibonacci::run_fibonacci();
    } else {
        warn!("This program should be built in release mode for accurate benchmarking");
        warn!("Skipping fibonacci benchmark");
        println!();
    }
}
