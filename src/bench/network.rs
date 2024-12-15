use futures::{StreamExt,executor::block_on};
use std::time::Instant;

// Reverse engineering of speed.cloudflare.com
// I don't think cloudflare would mind, but if you do, please contact me :(

// WHAT THE FUCKING CODE IS THIS?
// DO YOU WANT TO FILL UP ALL MY MEMORY?
// AH WELL IT WORKS, I GUESS
async fn perform_speedtest() {

    let url = "https://speed.cloudflare.com/__down?bytes=1000000000";
    let mut log = paris::Logger::new();
    log.loading("Running speedtest to Cloudflare network...");
    let start_time = Instant::now();

    let mut stream = reqwest::get(url).await.unwrap().bytes_stream();
    let mut total_bytes = 0u64;

    while let Some(chunk) = stream.next().await {
        total_bytes += chunk.unwrap().len() as u64;
    }

    let duration = start_time.elapsed();
    let speed_mbps = (total_bytes as f64 * 8.0) / (duration.as_secs_f64() * 1_000_000.0);

    log.success(format!(
        "Downloaded {} bytes in {:.2} seconds ({:.2} Mbps)",
        total_bytes, duration.as_secs_f64(), speed_mbps
    ));
    
}

pub fn start_speedtest() {
    block_on(perform_speedtest());
}
