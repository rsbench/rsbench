use futures::{executor::block_on, StreamExt};
use std::time::Instant;

// Reverse engineering of speed.cloudflare.com
// I don't think cloudflare would mind, but if you do, please contact me :(
async fn perform_speedtest() {
    let url = "https://speed.cloudflare.com/__down?bytes=10000000000";
    let mut log = paris::Logger::new();
    log.loading("Running speedtest to Cloudflare network...");
    let start_time = Instant::now();

    let mut stream = reqwest::get(url).await.unwrap().bytes_stream();
    let mut total_bytes = 0u64;
    let mut max_speed_mbps = 0f64;
    let last_chunk = Instant::now();
    while let Some(chunk) = stream.next().await {
        total_bytes += chunk.unwrap().len() as u64;
        let elapsed = last_chunk.elapsed().as_secs_f64();
        if elapsed >= 0.5 {
            let speed = (total_bytes as f64 * 8.0) / (elapsed * 1_000_000.0);
            if speed > max_speed_mbps {
                max_speed_mbps = speed;
            }
        }
        if start_time.elapsed().as_secs() >= 10 {
            break;
        }
    }

    let duration = start_time.elapsed();
    let speed_mbps = (total_bytes as f64 * 8.0) / (duration.as_secs_f64() * 1_000_000.0);
    log.done();
    println!(
        "DOWN: 🔽 {:.2} Mbps | MAX {:.2} Mbps",
        speed_mbps, max_speed_mbps
    );
}

pub fn start_speedtest() {
    block_on(perform_speedtest());
}
