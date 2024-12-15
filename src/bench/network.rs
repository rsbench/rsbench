use futures::{StreamExt, executor::block_on};
use std::time::Instant;

async fn perform_speedtest() -> (f64, Vec<f64>) {
    let url = "https://speed.cloudflare.com/__down?bytes=10000000000";
    let start_time = Instant::now();

    let mut stream = reqwest::get(url).await.unwrap().bytes_stream();
    let mut speed_samples = Vec::new();
    let mut interval_bytes = 0u64;
    let mut last_time = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk_len = chunk.unwrap().len() as u64;
        interval_bytes += chunk_len;

        let elapsed = last_time.elapsed().as_secs_f64();
        if elapsed >= 0.5 {
            let speed = (interval_bytes as f64 * 8.0) / (elapsed * 1_000_000.0); // Mbps
            speed_samples.push(speed);
            interval_bytes = 0;
            last_time = Instant::now();
        }

        if start_time.elapsed().as_secs() >= 10 {
            break;
        }
    }

    let mean_speed = if !speed_samples.is_empty() {
        speed_samples.iter().sum::<f64>() / speed_samples.len() as f64
    } else {
        0.0
    };

    (mean_speed, speed_samples)
}

pub fn start_speedtest() {
    //let rt = tokio::runtime::Runtime::new().unwrap();
    let mut log = paris::Logger::new();
    log.loading("Running single thread download test...");
    let (mean_speed_mbps, speed_samples) = block_on(perform_speedtest());
    let max = speed_samples.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    log.done();
    println!("DOWN: 🔽 {:.2} Mbps | MAX : {:.2} Mbps", mean_speed_mbps, max);
}

pub fn start_multithread_speedtest(num_concurrent: usize) {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    let mut log = paris::Logger::new();
    log.loading("Running multiple thread download test...");

    let results = block_on(async {
        let mut handles = Vec::new();
        for _ in 0..num_concurrent {
            handles.push(tokio::spawn(async {
                perform_speedtest().await
            }));
        }
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap());
        }
        results
    });

    let total_mean_speed: f64 = results.iter().map(|(mean_speed, _)| mean_speed).sum();
    

    let mut all_speed_samples: Vec<f64> = Vec::new();
    for (_, speed_samples) in &results {
        all_speed_samples.extend(speed_samples);
    }

    let mut instant_speeds: Vec<f64> = Vec::new();
    for i in 0..all_speed_samples.len() {
        let mut instant_speed = 0.0;
        for j in 0..results.len() {
            if i < results[j].1.len() {
                instant_speed += results[j].1[i];
            }
        }
        instant_speeds.push(instant_speed);
    }
    let max = instant_speeds.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    log.done();

    println!("DOWN: ⏬ {:.2} Mbps | MAX : {:.2} Mbps", total_mean_speed, max);
}
