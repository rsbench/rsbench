mod cloudflare;
mod ip_sb;
mod ipcheck_ing;
mod ipinfo_io;
mod ipip_net;
mod ipquery_io;
mod myip_la;
mod pconline;
mod utils;
mod vore_api;

use crate::tune::ip_check::utils::format_center;
use crate::utils::{clear_last_line, set_colour, set_default_colour, set_random_colour};
use async_trait::async_trait;
use prettytable::{color, format, Attr, Cell, Row, Table};
use std::fmt::Display;
use std::net::IpAddr;
use termcolor::Color;
use tokio::sync::mpsc;

#[derive(Debug, Default)]
pub struct IPCheckProviderV4 {
    pub provider: String,
    pub success: bool,
    pub ip: Option<IpAddr>,
    pub org: Option<String>,
    pub region: Option<String>,
    pub risk_score: Option<u8>,
}
#[derive(Debug, Default)]
pub struct IPCheckProviderV6 {
    pub provider: String,
    pub success: bool,
    pub ip: Option<IpAddr>,
    pub org: Option<String>,
    pub region: Option<String>,
    pub risk_score: Option<u8>,
}

#[async_trait]
#[allow(dead_code)]
pub(crate) trait IPCheck {
    fn provider_name(&self) -> String;

    async fn check(&self) -> (IPCheckProviderV4, IPCheckProviderV6);
}

pub async fn ip_all() {
    set_colour(Color::Yellow);
    println!("IP  :");
    println!("{:^10} | {:^40} ", "Provider", "IP",);
    set_default_colour();

    let provider_list: Vec<Box<dyn IPCheck + Send + Sync>> = vec![
        Box::new(ipinfo_io::IpInfoIo),
        Box::new(ipquery_io::IPQueryIo),
        Box::new(ipip_net::IpIpNet),
        Box::new(myip_la::MyIpLa),
        Box::new(cloudflare::Cloudflare),
        Box::new(ipcheck_ing::IPCheckIng),
        Box::new(pconline::PcOnline),
        Box::new(vore_api::VoreAPI),
        Box::new(ip_sb::IpSb),
    ];

    let (tx, mut rx) = mpsc::channel(100);

    let time = tokio::time::Instant::now();

    for provider in provider_list {
        let tx = tx.clone();
        tokio::spawn(async move {
            let result = provider.check().await;
            tx.send(result).await.unwrap();
        });
    }

    drop(tx);

    let mut results_v4 = Vec::new();
    let mut results_v6 = Vec::new();
    while let Some(provider) = rx.recv().await {
        let provider_v4 = provider.0;
        print!("{provider_v4}");
        let provider_v6 = provider.1;
        print!("{provider_v6}");
        results_v4.push(provider_v4);
        results_v6.push(provider_v6);
    }

    let time = time.elapsed().as_secs_f32();

    let results_count = results_v4.iter().map(|r| r.success).filter(|&s| s).count()
        + results_v6.iter().map(|r| r.success).filter(|&s| s).count();

    for _ in 0..=results_count {
        clear_last_line();
    }

    let table = get_table(results_v4, results_v6).await;
    table.printstd();

    set_colour(Color::Yellow);
    println!("Tested {results_count} projects took {time:.2} seconds",);
}

impl Display for IPCheckProviderV4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.success {
            set_random_colour();
            writeln!(
                f,
                "{} | {}",
                format_center(&self.provider, 10),
                format_center(&self.ip.unwrap().to_string(), 40),
            )
        } else {
            write!(f, "")
        }
    }
}

impl Display for IPCheckProviderV6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.success {
            set_random_colour();
            writeln!(
                f,
                "{} | {}",
                format_center(&self.provider, 10),
                format_center(&self.ip.unwrap().to_string(), 40),
            )
        } else {
            write!(f, "")
        }
    }
}

async fn get_table(v4: Vec<IPCheckProviderV4>, v6: Vec<IPCheckProviderV6>) -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(vec![
        Cell::new("Provider")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("IP")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Region")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Risk")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
        Cell::new("Org")
            .with_style(Attr::ForegroundColor(color::YELLOW))
            .with_style(Attr::Bold),
    ]));

    for provider in v4 {
        match provider.success {
            false => continue,
            true => {
                table.add_row(Row::new(vec![
                    Cell::new(&provider.provider).with_style(Attr::ForegroundColor(color::BLUE)),
                    Cell::new(&provider.ip.unwrap().to_string())
                        .with_style(Attr::ForegroundColor(color::GREEN)),
                    Cell::new(provider.region.as_ref().unwrap_or(&"N/A".to_string()))
                        .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                    match provider.risk_score {
                        Some(risk_score) => Cell::new(&risk_score.to_string())
                            .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
                        None => Cell::new("N/A")
                            .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
                    },
                    Cell::new(provider.org.as_ref().unwrap_or(&"N/A".to_string()))
                        .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE)),
                ]));
            }
        }
    }
    for provider in v6 {
        match provider.success {
            false => continue,
            true => {
                table.add_row(Row::new(vec![
                    Cell::new(&provider.provider).with_style(Attr::ForegroundColor(color::BLUE)),
                    Cell::new(&provider.ip.unwrap().to_string())
                        .with_style(Attr::ForegroundColor(color::GREEN)),
                    Cell::new(provider.region.as_ref().unwrap_or(&"N/A".to_string()))
                        .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                    match provider.risk_score {
                        Some(risk_score) => Cell::new(&risk_score.to_string())
                            .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
                        None => Cell::new("N/A")
                            .with_style(Attr::ForegroundColor(color::BRIGHT_MAGENTA)),
                    },
                    Cell::new(provider.org.as_ref().unwrap_or(&"N/A".to_string()))
                        .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE)),
                ]));
            }
        }
    }

    table
}
