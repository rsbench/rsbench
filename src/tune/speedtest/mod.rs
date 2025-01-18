use crate::tune::speedtest::single::{single_download, single_upload};
use crate::utils::{set_colour, set_default_colour, set_random_colour};
use futures::executor::block_on;
use prettytable::{color, format, Attr, Cell, Row, Table};
use termcolor::Color;

mod multi;
mod single;

const CN_CM: &str = "speedtest1.sc.chinamobile.com:8080";
const _CN_CM2: &str = "speedtest.139play.com:8080";
const CN_CU: &str = "36.250.1.90:8080";
const CN_CT: &str = "speedtest1.online.sh.cn:8080";

pub fn run_single() -> (
    Vec<(String, f64, f64, Vec<f64>)>,
    Vec<(String, f64, f64, Vec<f64>)>,
) {
    let mut log = paris::Logger::new();

    let providers = vec![
        ("China Mobile", CN_CM),
        ("China Unicom", CN_CU),
        ("China Telecom", CN_CT),
    ];

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
        let url = format!("http://{}/download?size=1000000000", host);
        log.loading(&format!(
            "Running single thread downloading test for \"{}\"",
            name
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
        let url = format!("http://{}/upload", host);
        log.loading(&format!(
            "Running single thread uploading test for \"{}\"",
            name
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

pub fn run_multi() -> (
    Vec<(String, f64, f64, Vec<Vec<f64>>)>,
    Vec<(String, f64, f64, Vec<Vec<f64>>)>,
) {
    println!();
    let mut log = paris::Logger::new();

    let providers = vec![
        ("China Mobile", CN_CM),
        ("China Unicom", CN_CU),
        ("China Telecom", CN_CT),
    ];

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
        let url = format!("http://{}/download?size=1000000000", host);
        log.loading(&format!(
            "Running multi thread downloading test for \"{}\"",
            name
        ));
        let (avg_speed, max_speed, speeds) = block_on(multi::multi_download(url.as_str(), 4));
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
        let url = format!("http://{}/upload", host);
        log.loading(&format!(
            "Running multi thread uploading test for \"{}\"",
            name
        ));
        let (avg_speed, max_speed, speeds) = block_on(multi::multi_upload(url.as_str(), 4));
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

pub fn get_table(
    single_download_results: Vec<(String, f64, f64, Vec<f64>)>,
    single_upload_results: Vec<(String, f64, f64, Vec<f64>)>,
    multi_download_results: Vec<(String, f64, f64, Vec<Vec<f64>>)>,
    multi_upload_results: Vec<(String, f64, f64, Vec<Vec<f64>>)>,
) -> (Table, Table) {
    let (single, multi) = parse_result(
        single_download_results,
        single_upload_results,
        multi_download_results,
        multi_upload_results,
    );

    let mut table_single = Table::new();
    table_single.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table_single.set_titles(Row::new(vec![
        Cell::new("Provider")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Avg Down")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Max Down")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Avg Up")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Max Up")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
    ]));
    for (name, avg_down, max_down, avg_up, max_up) in single {
        table_single.add_row(Row::new(vec![
            Cell::new(&name).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new(&format!("{:.2}", avg_down))
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(&format!("{:.2}", max_down))
                .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
            Cell::new(&format!("{:.2}", avg_up))
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(&format!("{:.2}", max_up))
                .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
        ]));
    }

    let mut table_multi = Table::new();
    table_multi.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table_multi.set_titles(Row::new(vec![
        Cell::new("Provider")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Avg Down")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Max Down")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Avg Up")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Max Up")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
    ]));
    for (name, avg_down, max_down, avg_up, max_up) in multi {
        table_multi.add_row(Row::new(vec![
            Cell::new(&name).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new(&format!("{:.2}", avg_down))
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(&format!("{:.2}", max_down))
                .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
            Cell::new(&format!("{:.2}", avg_up))
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(&format!("{:.2}", max_up))
                .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
        ]));
    }

    (table_single, table_multi)
}

fn parse_result(
    single_download_results: Vec<(String, f64, f64, Vec<f64>)>,
    single_upload_results: Vec<(String, f64, f64, Vec<f64>)>,
    multi_download_results: Vec<(String, f64, f64, Vec<Vec<f64>>)>,
    multi_upload_results: Vec<(String, f64, f64, Vec<Vec<f64>>)>,
) -> (
    Vec<(String, f64, f64, f64, f64)>,
    Vec<(String, f64, f64, f64, f64)>,
) {
    let mut combined_results_single = Vec::new();
    for i in 0..single_download_results.len() {
        let (download_string, download_f64_avg, download_f64_max, _) = &single_download_results[i];
        let (upload_string, upload_f64_avg, upload_f64_max, _) = &single_upload_results[i];

        // 确保 String 的值是一样的，如果不一样，你需要考虑如何处理这种情况
        assert_eq!(download_string, upload_string);

        combined_results_single.push((
            download_string.clone(),
            download_f64_avg.clone(),
            download_f64_max.clone(),
            upload_f64_avg.clone(),
            upload_f64_max.clone(),
        ));
    }

    let mut combined_results_multi = Vec::new();
    for i in 0..multi_download_results.len() {
        let (download_string, download_f64_avg, download_f64_max, _) = &multi_download_results[i];
        let (upload_string, upload_f64_avg, upload_f64_max, _) = &multi_upload_results[i];

        assert_eq!(download_string, upload_string);

        combined_results_multi.push((
            download_string.clone(),
            download_f64_avg.clone(),
            download_f64_max.clone(),
            upload_f64_avg.clone(),
            upload_f64_max.clone(),
        ));
    }
    (combined_results_single, combined_results_multi)
}
