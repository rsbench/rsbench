use crate::tune::ip_check::utils::create_reqwest_client;
use crate::tune::ip_check::{IPCheck, IPCheckProvider};
use async_trait::async_trait;
use reqwest::Response;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct Cloudflare;

#[async_trait]
impl IPCheck for Cloudflare {
    fn provider_name(&self) -> String {
        "Cloudflare".to_string()
    }

    async fn check(&self) -> IPCheckProvider {
        let handle_v4 = tokio::spawn(async move {
            let client_v4 = match create_reqwest_client(Some("curl/8.11.1"), false).await {
                Ok(client) => client,
                Err(()) => {
                    return None;
                }
            };

            let result = match client_v4.get("https://1.0.0.1/cdn-cgi/trace").send().await {
                Ok(result) => result,
                Err(_) => {
                    return None;
                }
            };
            parse_cloudflare(result).await
        });

        let handle_v6 = tokio::spawn(async move {
            let client_v6 = match create_reqwest_client(Some("curl/8.11.1"), true).await {
                Ok(client) => client,
                Err(()) => {
                    return None;
                }
            };

            let result = match client_v6
                .get("https://[2606:4700:4700::1111]/cdn-cgi/trace")
                .send()
                .await
            {
                Ok(result) => result,
                Err(_) => {
                    return None;
                }
            };
            parse_cloudflare(result).await
        });

        let ip_v4 = handle_v4.await.unwrap_or(None);
        let ip_v6 = handle_v6.await.unwrap_or(None);

        let mut response = IPCheckProvider {
            provider: self.provider_name(),
            success: true,
            ipv4: ip_v4,
            ipv4_org: None,
            ipv4_region: None,
            ipv6: ip_v6,
            ipv6_org: None,
            ipv6_region: None,
            risk_score_v4: None,
            risk_score_v6: None,
        };

        if ip_v4.is_none() {
            response.success = false;
        }

        response
    }
}

async fn parse_cloudflare(response: Response) -> Option<IpAddr> {
    if !response.status().is_success() {
        return None;
    }
    let html = match response.text().await {
        Ok(html) => html,
        Err(_) => {
            return None;
        }
    };

    let mut ip = String::new();
    for line in html.lines() {
        if line.starts_with("ip=") {
            ip = line.split("=").collect::<Vec<&str>>()[1].to_string();
            break;
        }
    }

    let ip = match Ipv4Addr::from_str(ip.as_str()) {
        Ok(ip) => IpAddr::V4(ip),
        Err(_) => match Ipv6Addr::from_str(ip.as_str()) {
            Ok(ip) => IpAddr::V6(ip),
            Err(_) => {
                return None;
            }
        },
    };
    Some(ip)
}
