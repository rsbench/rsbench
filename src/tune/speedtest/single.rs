use crate::tune::speedtest::get_providers;
use crate::utils::{set_colour, set_default_colour, set_random_colour};
use async_stream::stream;
use futures::executor::block_on;
use futures::StreamExt;
use reqwest::Client;
use std::time::Duration;
use termcolor::Color;
use tokio::time::Instant;

pub async fn single_download(url: &str) -> (f64, f64, Vec<f64>) {
    // (avg_speed, max_speed, speeds)
    let client = Client::builder().user_agent("curl/7.64.1").build().unwrap();

    let Ok(response) = client.get(url).send().await else {
        return (0.0, 0.0, Vec::new());
    };

    if !response.status().is_success() {
        return (0.0, 0.0, Vec::new());
    }

    let mut stream = response.bytes_stream();
    let mut speeds = Vec::new();
    let mut total_bytes_read = 0;
    let start_time = Instant::now();
    let mut last_check_time = start_time;

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                total_bytes_read += bytes.len();
                let now = Instant::now();
                if now.duration_since(last_check_time).as_secs() >= 1 {
                    let elapsed = now.duration_since(start_time).as_secs_f64();
                    let speed = total_bytes_read as f64 / elapsed; // in bytes per second
                    speeds.push(speed / 1024.0 / 1024.0 * 8.0); // in Mbps
                    last_check_time = now;
                }
                if start_time.elapsed().as_secs_f64() >= 15.0 {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    let duration = start_time.elapsed();
    let average_speed = total_bytes_read as f64 / duration.as_secs_f64() / 1024.0 / 1024.0 * 8.0;

    let max = speeds
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);

    (average_speed, *max, speeds)
}

pub async fn single_upload(url: &str) -> (f64, f64, Vec<f64>) {
    // (avg_speed, max_speed, speeds)
    let client = Client::builder().user_agent("curl/7.64.1").build().unwrap();

    let mut speeds = Vec::new();
    let start_time = Instant::now();
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let body = reqwest::Body::wrap_stream(stream! {
        let mut interval = 0u64;
        let mut last_time = Instant::now();
        while start_time.elapsed().as_secs() < 15 {
            let elapsed = last_time.elapsed().as_secs_f64();
            if elapsed >= 0.5 {
                let speed = (interval as f64 * 8.0) / (elapsed * 1_000_000.0); // Mbps
                tx.try_send(speed).unwrap();
                interval = 0;
                last_time = Instant::now();
            }
            interval += 1024;
            yield Ok::<Vec<u8>, reqwest::Error>(vec![0u8; 1024]);
        }
    });

    let _ = client
        .post(url)
        .body(body)
        .timeout(Duration::from_secs(15))
        .send()
        .await;

    rx.recv_many(&mut speeds, u16::MAX as usize).await;

    let average_speed = speeds.iter().sum::<f64>() / speeds.len() as f64;
    let max = speeds
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);

    (average_speed, *max, speeds)
}

type SingleUpOrDown = Vec<(String, f64, f64, Vec<f64>)>;

pub fn run_single() -> (SingleUpOrDown, SingleUpOrDown) {
    let mut log = paris::Logger::new();

    let providers = get_providers();

    let mut download_results = Vec::new();
    set_colour(Color::Yellow);
    print!("Running single thread download test for the following providers:");
    println!(
        "\n{:^15} | {:^15} | {:^15}",
        "Provider", "Avg Speed", "Max Speed"
    );
    set_default_colour();
    for provider in providers.clone() {
        let (name, host) = provider;
        let url = format!("http://{host}/download?size=1000000000");
        log.loading(format!(
            "Running single thread downloading test for \"{name}\""
        ));
        let (avg_speed, max_speed, speeds) = block_on(single_download(url.as_str()));
        log.done();
        download_results.push((name.to_string(), avg_speed, max_speed, speeds));

        set_random_colour();
        print!("{name:^15}");
        set_random_colour();
        print!(" | {avg_speed:^10.2} Mbps");
        set_random_colour();
        println!(" | {max_speed:^10.2} Mbps");
    }

    let mut upload_results = Vec::new();
    set_colour(Color::Yellow);
    println!();
    print!("Running single thread upload test for the following providers:");
    println!(
        "\n{:^15} | {:^15} | {:^15}",
        "Provider", "Avg Speed", "Max Speed"
    );
    for provider in providers.clone() {
        let (name, host) = provider;
        let url = format!("http://{host}/upload");
        log.loading(format!(
            "Running single thread uploading test for \"{name}\""
        ));
        let (avg_speed, max_speed, speeds) = block_on(single_upload(url.as_str()));
        log.done();
        upload_results.push((name.to_string(), avg_speed, max_speed, speeds));

        set_random_colour();
        print!("{name:^15}");
        set_random_colour();
        print!(" | {avg_speed:^10.2} Mbps");
        set_random_colour();
        println!(" | {max_speed:^10.2} Mbps");
    }

    (download_results, upload_results)
}
