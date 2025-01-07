mod ipinfo_io;
mod ipip_net;
mod ipquery_io;
mod myip_la;
mod utils;

use crate::tune::ip_check::utils::format_center;
use crate::utils::{set_colour, set_default_colour, set_random_colour};
use async_trait::async_trait;
use std::fmt::Display;
use std::net::IpAddr;
use termcolor::Color;
use tokio::sync::mpsc;

#[derive(Debug, Default)]
pub struct IPCheckProvider {
    pub provider: String,
    pub success: bool,
    pub ipv4: Option<IpAddr>,
    pub ipv4_org: Option<String>,
    pub ipv4_region: Option<String>,
    pub ipv6: Option<IpAddr>,
    pub ipv6_org: Option<String>,
    pub ipv6_region: Option<String>,
    pub risk_score_v4: Option<u8>,
    pub risk_score_v6: Option<u8>,
}

#[async_trait]
#[allow(dead_code)]
pub(crate) trait IPCheck {
    fn provider_name(&self) -> String;

    async fn check(&self) -> IPCheckProvider;
}

pub async fn ip_all() {
    set_colour(Color::Yellow);
    println!("IP  :");
    println!(
        "{:^10} | {:^40} | {:^30} | {:^5} | Org",
        "Provider", "IP", "Region", "Risk"
    );
    set_default_colour();

    let provider_list: Vec<Box<dyn IPCheck + Send + Sync>> = vec![
        Box::new(ipinfo_io::IpInfoIo),
        Box::new(ipquery_io::IPQueryIo),
        Box::new(ipip_net::IpIpNet),
        Box::new(myip_la::MyIpLa),
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

    let mut results = Vec::new();
    while let Some(provider) = rx.recv().await {
        print!("{provider}");
        results.push(provider);
    }
    let time = time.elapsed().as_secs_f32();
    let results_count = results.len();

    println!("Tested {results_count} projects took {time:.2} seconds",);
}

impl Display for IPCheckProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ipv4.is_some() {
            set_random_colour();
            write!(f, "{:^10}", self.provider)?;
            match &self.ipv4 {
                Some(ip) => {
                    write!(f, "   {ip:^40}")?;
                }
                None => {
                    write!(f, "   {:^40}", "N/A")?;
                }
            }
            match &self.ipv4_region {
                Some(region) => {
                    let region = format_center(region, 30);
                    write!(f, "   {region}")?;
                }
                None => {
                    write!(f, "   {:^20}", "N/A")?;
                }
            }
            match &self.risk_score_v4 {
                Some(risk) => {
                    write!(f, "   {risk:^5}")?;
                }
                None => {
                    write!(f, "   {:^5}", "N/A")?;
                }
            }
            match &self.ipv4_org {
                Some(org) => {
                    write!(f, "   {org}")?;
                }
                None => {
                    write!(f, "   N/A")?;
                }
            }
            writeln!(f)?;
        }

        if self.ipv6.is_some() {
            set_random_colour();
            write!(f, "{:^10}", self.provider)?;
            match &self.ipv6 {
                Some(ip) => {
                    write!(f, "   {ip:^40}")?;
                }
                None => {
                    write!(f, "   {:^40}", "N/A")?;
                }
            }
            match &self.ipv6_region {
                Some(region) => {
                    let region = format_center(region, 30);
                    write!(f, "   {region}")?;
                }
                None => {
                    write!(f, "   {:^20}", "N/A")?;
                }
            }
            match &self.risk_score_v6 {
                Some(risk) => {
                    write!(f, "   {risk:^5}")?;
                }
                None => {
                    write!(f, "   {:^5}", "N/A")?;
                }
            }
            match &self.ipv6_org {
                Some(org) => {
                    write!(f, "   {org}")?;
                }
                None => {
                    write!(f, "   N/A")?;
                }
            }
            writeln!(f)?;
        }
        set_default_colour();
        write!(f, "")
    }
}
