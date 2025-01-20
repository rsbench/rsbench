use crate::tune::speedtest::get_providers;
use crate::tune::speedtest::single::{single_download, single_upload};
use crate::utils::color::{set_colour, set_default_colour, set_random_colour};
use futures::executor::block_on;
use termcolor::Color;

pub async fn multi_download(url: &str, thread_count: u8) -> (f64, f64, Vec<Vec<f64>>) {
    let mut handles = Vec::new();
    for _ in 0..thread_count {
        let url = url.to_string();
        let handle = tokio::spawn(async move {
            let (avg_speed, max_speed, speeds) = single_download(url.as_str()).await;
            (avg_speed, max_speed, speeds)
        });
        handles.push(handle);
    }
    let mut avg_speeds = Vec::new();
    let mut speeds = Vec::new();
    for handle in handles {
        if let Ok((avg_speed, _max_speed, speed)) = handle.await {
            avg_speeds.push(avg_speed);
            speeds.push(speed);
        } else {
            avg_speeds.push(0.0);
            speeds.push(Vec::new());
        };
    }
    let avg_speed = avg_speeds.iter().sum();

    let max_len = speeds.iter().map(std::vec::Vec::len).max().unwrap_or(0);
    let mut speeds_sum = vec![0.0; max_len];
    for (i, _) in speeds_sum.clone().iter_mut().enumerate().take(max_len) {
        for inner_vec in &speeds {
            if let Some(&value) = inner_vec.get(i) {
                speeds_sum[i] += value;
            }
        }
    }

    let max = speeds_sum
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);

    (avg_speed, *max, speeds)
}

pub async fn multi_upload(url: &str, thread_count: u8) -> (f64, f64, Vec<Vec<f64>>) {
    let mut handles = Vec::new();
    for _ in 0..thread_count {
        let url = url.to_string();
        let handle = tokio::spawn(async move {
            let (avg_speed, max_speed, speeds) = single_upload(url.as_str()).await;
            (avg_speed, max_speed, speeds)
        });
        handles.push(handle);
    }
    let mut avg_speeds = Vec::new();
    let mut speeds = Vec::new();
    for handle in handles {
        if let Ok((avg_speed, _max_speed, speed)) = handle.await {
            avg_speeds.push(avg_speed);
            speeds.push(speed);
        } else {
            avg_speeds.push(0.0);
            speeds.push(Vec::new());
        };
    }
    let avg_speed = avg_speeds.iter().sum();

    let max_len = speeds.iter().map(std::vec::Vec::len).max().unwrap_or(0);
    let mut speeds_sum = vec![0.0; max_len];
    for (i, _) in speeds_sum.clone().iter_mut().enumerate().take(max_len) {
        for inner_vec in &speeds {
            if let Some(&value) = inner_vec.get(i) {
                speeds_sum[i] += value;
            }
        }
    }

    let max = speeds_sum
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(&0.0);

    (avg_speed, *max, speeds)
}

type MultiUpOrDown = Vec<(String, f64, f64, Vec<Vec<f64>>)>;

pub fn run_multi() -> (MultiUpOrDown, MultiUpOrDown) {
    println!();
    let mut log = paris::Logger::new();

    let providers = get_providers();

    let mut download_results = Vec::new();
    set_colour(Color::Yellow);
    print!("Running multi thread download test for the following providers:");
    println!(
        "\n{:^15} | {:^15} | {:^15}",
        "Provider", "Avg Speed", "Max Speed"
    );
    set_default_colour();
    for provider in providers.clone() {
        let (name, host) = provider;
        let url = format!("http://{host}/download?size=1000000000");
        log.loading(format!(
            "Running multi thread downloading test for \"{name}\""
        ));
        let (avg_speed, max_speed, speeds) = block_on(multi_download(url.as_str(), 4));
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
    print!("Running multi thread upload test for the following providers:");
    println!(
        "\n{:^15} | {:^10} | {:^10}",
        "Provider", "Avg Speed", "Max Speed"
    );
    for provider in providers.clone() {
        let (name, host) = provider;
        let url = format!("http://{host}/upload");
        log.loading(format!(
            "Running multi thread uploading test for \"{name}\""
        ));
        let (avg_speed, max_speed, speeds) = block_on(multi_upload(url.as_str(), 4));
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
