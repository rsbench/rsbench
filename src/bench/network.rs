// use std::sync::mpsc::{Receiver, Sender};
use crate::utils::{set_colour, set_default_colour};
use async_stream::stream;
use futures::{
    channel::mpsc::{Receiver, Sender},
    executor::block_on,
    StreamExt,
};
use paris::error;
use std::time::{Duration, Instant};
use termcolor::Color;

const PING_TIMES: usize = 50;

struct TcpConnectTimedOut;

async fn perform_ping() -> Result<u128, TcpConnectTimedOut> {
    let addr = "1.1.1.1:443";
    let start = Instant::now();
    let stream = tokio::net::TcpStream::connect(addr);
    match tokio::time::timeout(Duration::from_secs(1), stream).await {
        Ok(Ok(stream)) => {
            let _ = stream.peer_addr();
            let _ = stream.local_addr();
            let elapsed = start.elapsed().as_millis();
            Ok(elapsed)
        }
        Ok(Err(_)) => Err(TcpConnectTimedOut),
        Err(_) => Err(TcpConnectTimedOut),
    }
}

pub fn ping() {
    let mut log = paris::Logger::new();
    log.loading("Running PING test...");
    let mut timeouts = 0;
    let mut results = Vec::new();

    for _ in 0..PING_TIMES {
        let result = block_on(perform_ping());
        match result {
            Ok(time) => results.push(time),
            Err(_) => timeouts += 1,
        }
    }
    log.done();
    let mean = results.iter().sum::<u128>() as f64 / results.len() as f64;
    let loss = (timeouts as f64 / PING_TIMES as f64) * 100.0;
    set_colour(Color::Blue);
    println!("PING: {mean:.2} ms");
    println!("LOSS: {loss:.2}%");
    set_default_colour();
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
    let (tx, mut rx): (Sender<f64>, Receiver<f64>) = futures::channel::mpsc::channel(20);
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

async fn perform_download() -> Result<(f64, Vec<f64>), String> {
    let url = "https://speed.cloudflare.com/__down?bytes=10000000000";
    let start_time = Instant::now();

    let mut stream = match reqwest::get(url).await {
        Ok(stream) => stream,
        Err(_) => {
            return Err(String::from(
                "An error occurred while connecting to the Cloudflare speed test server",
            ))
        }
    }
    .bytes_stream();
    let mut speed_samples = Vec::new();
    let mut interval_bytes = 0u64;
    let mut last_time = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk_len = match chunk {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(String::from(
                    "An error occurred while connecting to the Cloudflare speed test server",
                ))
            }
        }
        .len() as u64;
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

    Ok((mean_speed, speed_samples))
}

pub fn start_speedtest() {
    let mut log = paris::Logger::new();
    log.loading("Running single thread download test...");
    let result = block_on(perform_download());
    let (mean_speed_mbps, speed_samples) = match result {
        Ok(tuple) => tuple,
        Err(error) => {
            log.done();
            println!();
            log.error(error);
            return;
        }
    };

    let max = speed_samples
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_else(|| {
            error!("Unable to find max speed");
            &0.0
        });
    log.done();

    let binding = mean_speed_mbps.to_string();
    let mean_speed_mbps_str = &binding.as_str()[0..5];

    let binding = max.to_string();
    let max_str = &binding.as_str()[0..5];

    set_colour(Color::Yellow);
    print!("DOWN: 🔽 ");
    set_colour(Color::Rgb(64, 224, 208));
    print!("{mean_speed_mbps_str} Mbps");
    set_colour(Color::Yellow);
    print!(" | ");
    set_colour(Color::Yellow);
    print!("MAX : ");
    set_colour(Color::Rgb(65, 105, 225));
    println!("{max_str} Mbps");
    set_default_colour();

    // Upload test
    log.loading("Running single thread upload test...");
    let (mean_speed_mbps, speed_samples) = block_on(perform_upload());
    let max = speed_samples
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_else(|| {
            error!("Unable to find max speed");
            &0.0
        });
    log.done();

    let binding = mean_speed_mbps.to_string();
    let mean_speed_mbps_str = &binding.as_str()[0..5];

    let binding = max.to_string();
    let max_str = &binding.as_str()[0..5];

    set_colour(Color::Yellow);
    print!("UP  : 🔼 ");
    set_colour(Color::Rgb(139, 0, 139));
    print!("{mean_speed_mbps_str} Mbps");
    set_colour(Color::Yellow);
    print!(" | ");
    set_colour(Color::Yellow);
    print!("MAX : ");
    set_colour(Color::Rgb(72, 61, 139));
    println!("{max_str} Mbps");
    set_default_colour();
}

pub fn start_multithread_speedtest(num_concurrent: usize) {
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

    let mut results_success = Vec::new();
    for result in results {
        match result {
            Ok(tuple) => results_success.push(tuple),
            Err(error) => {
                log.done();
                println!();
                log.error(error);
                return;
            }
        }
    }

    let total_mean_speed: f64 = results_success
        .iter()
        .map(|(mean_speed, _)| mean_speed)
        .sum();

    let mut all_speed_samples: Vec<f64> = Vec::new();
    for (_, speed_samples) in &results_success {
        all_speed_samples.extend(speed_samples);
    }

    let mut instant_speeds: Vec<f64> = Vec::new();
    for i in 0..all_speed_samples.len() {
        let mut instant_speed = 0.0;
        for result_success in &results_success {
            if i < result_success.1.len() {
                instant_speed += result_success.1[i];
            }
        }
        instant_speeds.push(instant_speed);
    }
    let max = instant_speeds
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_else(|| {
            error!("Unable to find max speed");
            &0.0
        });
    log.done();

    let binding = total_mean_speed.to_string();
    let total_mean_speed_str = &binding.as_str()[0..5];

    let binding = max.to_string();
    let max_str = &binding.as_str()[0..5];

    set_colour(Color::Yellow);
    print!("DOWN: ⏬ ");
    set_colour(Color::Rgb(64, 224, 208));
    print!("{total_mean_speed_str} Mbps");
    set_colour(Color::Yellow);
    print!(" | ");
    set_colour(Color::Yellow);
    print!("MAX : ");
    set_colour(Color::Rgb(65, 105, 225));
    println!("{max_str} Mbps");
    set_default_colour();

    // upload test
    log.loading("Running multiple thread upload test...");
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
        for result in &results {
            if i < result.1.len() {
                instant_speed += result.1[i];
            }
        }
        instant_speeds.push(instant_speed);
    }
    let max = instant_speeds
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_else(|| {
            error!("Unable to find max speed");
            &0.0
        });
    log.done();

    let binding = total_mean_speed.to_string();
    let total_mean_speed_str = &binding.as_str()[0..5];

    let binding = max.to_string();
    let max_str = &binding.as_str()[0..5];

    set_colour(Color::Yellow);
    print!("UP  : ⏫ ");
    set_colour(Color::Rgb(139, 0, 139));
    print!("{total_mean_speed_str} Mbps");
    set_colour(Color::Yellow);
    print!(" | ");
    set_colour(Color::Yellow);
    print!("MAX : ");
    set_colour(Color::Rgb(72, 61, 139));
    println!("{max_str} Mbps");
    set_default_colour();
}
