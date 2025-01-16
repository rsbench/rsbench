use crate::utils::{clear_last_line, set_colour, set_default_colour, set_random_colour};
use futures::StreamExt;
use paris::error;
use prettytable::{color, format, Attr, Cell, Row, Table};
use std::collections::HashMap;
use std::time::Instant;
use termcolor::Color;

const CN_CM: &str = "speedtest1.sc.chinamobile.com:8080";
const _CN_CM2: &str = "speedtest.139play.com:8080";
const CN_CU: &str = "36.250.1.90:8080";
const CN_CT: &str = "speedtest1.online.sh.cn:8080";
const CN_CBN: &str = "speedtest1.cqccn.com:8080";

async fn run_speedtest_download_with_url(url: &str) -> Result<(f64, Vec<f64>), String> {
    let client = match reqwest::Client::builder().user_agent("curl/7.64.1").build() {
        Ok(client) => client,
        Err(_) => {
            return Err(String::from(
                "An error occurred while creating the speedtest client",
            ))
        }
    };
    let url = format!("http://{url}/download?size=50000000");
    let start_time = Instant::now();
    let mut speed_samples = Vec::new();
    let mut interval_bytes = 0u64;

    let mut stream = match client.get(url).send().await {
        Ok(stream) => stream,
        Err(_) => {
            return Err(String::from(
                "An error occurred while connecting to the speedtest server",
            ))
        }
    }
    .bytes_stream();

    let mut last_time = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk_len = match chunk {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(String::from(
                    "An error occurred while connecting to the speedtest server",
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

        if start_time.elapsed().as_secs() >= 15 {
            break;
        }
    }

    let mean_speed = if speed_samples.is_empty() {
        0.0
    } else {
        speed_samples.iter().sum::<f64>() / speed_samples.len() as f64
    };

    Ok((mean_speed, speed_samples))
}

pub async fn single_thread_download() {
    let mut log = paris::Logger::new();

    let mut node_hashmap = HashMap::new();
    node_hashmap.insert("China Mobile", CN_CM);
    node_hashmap.insert("China Unicom", CN_CU);
    node_hashmap.insert("China Telecom", CN_CT);
    node_hashmap.insert("China Broadnet", CN_CBN);

    let mut result_hashmap = HashMap::new();

    for (name, url) in node_hashmap.iter() {
        log.loading(format!("Speedtest is running ({})...", name));
        let (speed, samples) = run_speedtest_download_with_url(url)
            .await
            .unwrap_or_else(|e| {
                error!("An error occurred while running the speedtest: {}", e);
                (0.0, Vec::new())
            });
        log.done();
        let max = samples.clone();
        let max = max
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or_else(|| {
                error!("Unable to find max speed");
                &0.0
            });
        result_hashmap.insert(name, (speed, max.clone()));
        print_download_output(name, speed, max).await;
    }

    for _ in 0..4 {
        clear_last_line();
    }

    let table = get_table(result_hashmap).await;
    table.printstd();
    set_default_colour();
}

async fn print_download_output(name: &str, speed: f64, max: &f64) {
    let binding = speed.to_string();
    let total_mean_speed_str = &binding.as_str()[0..6];

    let binding = max.to_string();
    let max_str = &binding.as_str()[0..6];

    set_random_colour();
    print!("{}: ", name);
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
}

async fn get_table(results: HashMap<&&str, (f64, f64)>) -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(vec![
        Cell::new("Area")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Max Speed")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Avg Speed")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
    ]));

    for (name, (speed, max)) in results.iter() {
        table.add_row(Row::new(vec![
            Cell::new(*name)
                .with_style(Attr::ForegroundColor(color::CYAN))
                .with_style(Attr::Bold),
            Cell::new(format!("{:.2}", max).as_str())
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::Bold),
            Cell::new(format!("{:.2}", speed).as_str())
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::Bold),
        ]));
    }

    table
}
