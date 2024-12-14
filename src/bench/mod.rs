mod fibonacci;
mod network;
pub fn run_bench() {
    network::start_speedtest();
    fibonacci::run_fibonacci();
}
