mod fibonacci;
mod network;
pub fn run_bench() {
    network::ping();
    network::start_speedtest();
    network::start_multithread_speedtest(4);
    fibonacci::run_fibonacci();
}
