use async_stream::stream;
use futures::{
    channel::mpsc::{Receiver, Sender},
    executor::block_on,
    StreamExt,
};
// use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;

pub fn ping() {
    let mut results = Vec::new();
    let addr = "1.1.1.1:443";
    for _ in 0..50 {
        let start = Instant::now();
        let stream = block_on(tokio::net::TcpStream::connect(addr));
        let elapsed = start.elapsed().as_millis();
        results.push(elapsed);
        if let Ok(stream) = &stream {
            // Do not remove, to prevent compiler optimization
            let _ = stream.peer_addr();
            let _ = stream.local_addr();
        }
    }
    let mean = results.iter().sum::<u128>() as f64 / results.len() as f64;
    println!("PING: {:.2} ms", mean);
}


pub fn upload_stream_provider(
    time: u64,
    mut tx: Sender<f64>,
) -> impl futures::Stream<Item = Result<Vec<u8>, std::io::Error>> {
    let start = Instant::now();
    stream! {
        let mut interval = 0u64;
        let mut last_time = Instant::now();
        while start.elapsed().as_secs() < time {
            let elapsed = last_time.elapsed().as_secs_f64();
            if elapsed >= 0.5 {
                let speed = (interval as f64 * 8.0) / (elapsed * 1_000_000.0); // Mbps
                tx.try_send(speed).unwrap();
                interval = 0;
                last_time = Instant::now();
            }
            interval += 1024;
            yield Ok(vec![0u8; 1024]);
        }

    }
}

async fn perform_upload() -> (f64, Vec<f64>) {
    let url = "https://speed.cloudflare.com/__up";
    let client = reqwest::Client::new();
    let (mut tx, mut rx): (Sender<f64>, Receiver<f64>) = futures::channel::mpsc::channel(20);
    let conn = client
        .post(url)
        .body(reqwest::Body::wrap_stream(upload_stream_provider(10, tx)));

    // Start collecting speed samples concurrently
    let speed_samples_task = tokio::spawn(async move {
        let mut speed_samples = Vec::new();
        while let Some(speed) = rx.next().await {
            speed_samples.push(speed);
        }
        speed_samples
    });

    // Send the request
    let _ = conn.send().await;

    // Wait for the speed samples to be collected
    let speed_samples = speed_samples_task.await.unwrap();

    // Calculate the mean speed
    let mean_speed = if !speed_samples.is_empty() {
        speed_samples.iter().sum::<f64>() / speed_samples.len() as f64
    } else {
        0.0
    };

    (mean_speed, speed_samples)
}

async fn perform_download() -> (f64, Vec<f64>) {
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
    let (mean_speed_mbps, speed_samples) = block_on(perform_download());
    let max = speed_samples
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    log.done();
    println!(
        "DOWN: 🔽 {:.2} Mbps | MAX : {:.2} Mbps",
        mean_speed_mbps, max
    );
    // Upload test
    let (mean_speed_mbps, speed_samples) = block_on(perform_upload());
    let max = speed_samples
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    println!(
        "UP  : 🔼 {:.2} Mbps | MAX : {:.2} Mbps",
        mean_speed_mbps, max
    );
}

pub fn start_multithread_speedtest(num_concurrent: usize) {
    // let rt = tokio::runtime::Runtime::new().unwrap();
    let mut log = paris::Logger::new();
    log.loading("Running multiple thread download test...");

    let results = block_on(async {
        let mut handles = Vec::new();
        for _ in 0..num_concurrent {
            handles.push(tokio::spawn(async { perform_download().await }));
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
    let max = instant_speeds
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    log.done();

    println!(
        "DOWN: ⏬ {:.2} Mbps | MAX : {:.2} Mbps",
        total_mean_speed, max
    );
    // upload test
    let results = block_on(async {
        let mut handles = Vec::new();
        for _ in 0..num_concurrent {
            handles.push(tokio::spawn(async { perform_upload().await }));
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
    let max = instant_speeds
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    println!(
        "UP  : ⏫ {:.2} Mbps | MAX : {:.2} Mbps",
        total_mean_speed, max
    );
}
