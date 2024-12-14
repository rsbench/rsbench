use futures::executor::block_on;
use std::time::Instant;

// Reverse engineering of speed.cloudflare.com
// I don't think cloudflare would mind, but if you do, please contact me :(
async fn perform_speedtest() {
    let url = "https://speed.cloudflare.com/__down?bytes=1000000000";
    let mut log = paris::Logger::new();
    log.loading("Running speedtest to Cloudflare network...");
    let start = Instant::now();

    let response = reqwest::get(url).await;

    match response {
        Ok(response) => {
            let bytes = response.bytes().await.unwrap();
            let duration = start.elapsed();
            log.done();
            let duration_secs = duration.as_secs_f64();
            let speed = bytes.len() as f64 / duration_secs;
            let speed_mbps = speed * 8.0 / 1_000_000.0;
            println!("DOWN: 🔽 {:.2} Mbps", speed_mbps);
        }
        Err(e) => println!("Failed to download file: {}", e),
    }
}

pub fn start_speedtest() {
    block_on(perform_speedtest());
}
