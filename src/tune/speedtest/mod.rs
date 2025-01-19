use crate::tune::speedtest::multi::run_multi;
use crate::tune::speedtest::single::run_single;
use crate::utils::{clear_last_line, set_colour, set_default_colour};
use prettytable::{color, format, Attr, Cell, Row, Table};
use reqwest::Client;
use serde_json::Value;
use termcolor::Color;

mod multi;
mod single;

pub fn run_speedtest_single_multi() {
    let (single_download, single_upload) = run_single();
    let (multi_download, multi_upload) = run_multi();

    for _ in 0..((get_providers().len() + 2 + 1) * 4 - 1) {
        clear_last_line();
    }

    let (table_single, table_multi) = get_table(
        &single_download,
        &single_upload,
        &multi_download,
        &multi_upload,
    );

    set_colour(Color::Yellow);
    println!("Single Thread Speedtest: ");
    set_default_colour();
    table_single.printstd();

    set_colour(Color::Yellow);
    println!();
    println!("Multi Thread Speedtest: ");
    set_default_colour();
    table_multi.printstd();
}

pub fn get_providers() -> Vec<(&'static str, String)> {
    if let Ok(custom) = std::env::var("CUSTOM_SPEEDTEST_SERVER") {
        vec![("Custom", custom)]
    } else {
        vec![
            ("Speedtest.net", {
                let host = futures::executor::block_on(get_speedtest_best_server());
                if let Ok(host) = host {
                    host
                } else {
                    "lg-lax.fdcservers.net.prod.hosts.ooklaserver.net:8080".to_string()
                }
            }),
            (
                "China Mobile",
                "speedtest1.sc.chinamobile.com:8080".to_string(),
            ),
            ("China Unicom", "36.250.1.90:8080".to_string()),
            ("China Telecom", "speedtest1.online.sh.cn:8080".to_string()),
            (
                "HK I3D",
                "hk.ap.speedtest.i3d.net.prod.hosts.ooklaserver.net:8080".to_string(),
            ),
            ("TW HiNet", "ntp1.chtm.hinet.net:8080".to_string()),
            (
                "JP xTom",
                "speedtest-kix.xtom.info.prod.hosts.ooklaserver.net:8080".to_string(),
            ),
        ]
    }
}

pub fn get_table(
    single_download_results: &[(String, f64, f64, Vec<f64>)],
    single_upload_results: &[(String, f64, f64, Vec<f64>)],
    multi_download_results: &[(String, f64, f64, Vec<Vec<f64>>)],
    multi_upload_results: &[(String, f64, f64, Vec<Vec<f64>>)],
) -> (Table, Table) {
    let last_result = parse_result(
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
    for (name, avg_down, max_down, avg_up, max_up) in last_result.single {
        table_single.add_row(Row::new(vec![
            Cell::new(&name).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new(&format!("{avg_down:.2}"))
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(&format!("{max_down:.2}"))
                .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
            Cell::new(&format!("{avg_up:.2}"))
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(&format!("{max_up:.2}"))
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
    for (name, avg_down, max_down, avg_up, max_up) in last_result.multi {
        table_multi.add_row(Row::new(vec![
            Cell::new(&name).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new(&format!("{avg_down:.2}"))
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(&format!("{max_down:.2}"))
                .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
            Cell::new(&format!("{avg_up:.2}"))
                .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
            Cell::new(&format!("{max_up:.2}"))
                .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
        ]));
    }

    (table_single, table_multi)
}

fn parse_result(
    single_download_results: &[(String, f64, f64, Vec<f64>)],
    single_upload_results: &[(String, f64, f64, Vec<f64>)],
    multi_download_results: &[(String, f64, f64, Vec<Vec<f64>>)],
    multi_upload_results: &[(String, f64, f64, Vec<Vec<f64>>)],
) -> LastResult {
    let mut combined_results_single = Vec::new();
    for i in 0..single_download_results.len() {
        let (download_string, download_f64_avg, download_f64_max, _) = &single_download_results[i];
        let (upload_string, upload_f64_avg, upload_f64_max, _) = &single_upload_results[i];

        // 确保 String 的值是一样的，如果不一样，你需要考虑如何处理这种情况
        assert_eq!(download_string, upload_string);

        combined_results_single.push((
            download_string.clone(),
            *download_f64_avg,
            *download_f64_max,
            *upload_f64_avg,
            *upload_f64_max,
        ));
    }

    let mut combined_results_multi = Vec::new();
    for i in 0..multi_download_results.len() {
        let (download_string, download_f64_avg, download_f64_max, _) = &multi_download_results[i];
        let (upload_string, upload_f64_avg, upload_f64_max, _) = &multi_upload_results[i];

        assert_eq!(download_string, upload_string);

        combined_results_multi.push((
            download_string.clone(),
            *download_f64_avg,
            *download_f64_max,
            *upload_f64_avg,
            *upload_f64_max,
        ));
    }
    LastResult {
        single: combined_results_single,
        multi: combined_results_multi,
    }
}

async fn get_speedtest_best_server() -> Result<String, String> {
    let mut log = paris::Logger::new();
    log.loading("Getting speedtest server list");
    let Ok(client) = Client::builder().user_agent("curl/7.64.1").build() else {
        log.done();
        return Err("Failed to create reqwest client".to_string());
    };

    let res = client
        .get("https://www.speedtest.net/api/js/servers?engine=js")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await;
    let json = if let Ok(res) = res {
        if let Ok(json) = res.json::<Value>().await {
            json
        } else {
            log.done();
            return Err("Failed to parse json".to_string());
        }
    } else {
        log.done();
        return Err("Failed to get speedtest server list".to_string());
    };

    let Some(array) = json.as_array() else {
        log.done();
        return Err("Failed to get speedtest server list".to_string());
    };

    let Some(best_host) = array.first() else {
        log.done();
        return Err("Failed to get speedtest server list".to_string());
    };

    let host = if let Some(host) = best_host.get("host") {
        if let Some(host) = host.as_str() {
            host
        } else {
            log.done();
            return Err("Failed to get speedtest server list".to_string());
        }
    } else {
        log.done();
        return Err("Failed to get speedtest server list".to_string());
    };
    log.done();

    Ok(host.parse().unwrap())
}

struct LastResult {
    single: Vec<(String, f64, f64, f64, f64)>,
    multi: Vec<(String, f64, f64, f64, f64)>,
}
