use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProviderV4, IPCheckProviderV6};
use async_trait::async_trait;
use reqwest::Response;
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct IpInfoIo;

#[async_trait]
impl IPCheck for IpInfoIo {
    fn provider_name(&self) -> String {
        "Ipinfo.io".to_string()
    }

    async fn check(&self) -> (IPCheckProviderV4, IPCheckProviderV6) {
        let handle_v4 = tokio::spawn(async move {
            let client_v4 = match create_reqwest_client(Some("curl/8.11.1"), false).await {
                Ok(client) => client,
                Err(()) => {
                    return (None, None, None);
                }
            };

            let result = match client_v4.get("https://ipinfo.io").send().await {
                Ok(result) => result,
                Err(_) => {
                    return (None, None, None);
                }
            };

            parse_ipinfo_json(result).await
        });

        let handle_v6 = tokio::spawn(async move {
            let client_v6 = match create_reqwest_client(Some("curl/8.11.1"), true).await {
                Ok(client) => client,
                Err(()) => {
                    return (None, None, None);
                }
            };

            let result = match client_v6.get("https://v6.ipinfo.io").send().await {
                Ok(result) => result,
                Err(_) => {
                    return (None, None, None);
                }
            };

            parse_ipinfo_json(result).await
        });

        let (ip_v4, org_v4, locate_v4) = handle_v4.await.unwrap_or((None, None, None));
        let (ip_v6, org_v6, locate_v6) = handle_v6.await.unwrap_or((None, None, None));

        let response_v4 = IPCheckProviderV4 {
            provider: self.provider_name(),
            success: ip_v4.is_some(),
            ip: ip_v4,
            org: org_v4,
            region: locate_v4,
            risk_score: None,
        };

        let response_v6 = IPCheckProviderV6 {
            provider: self.provider_name(),
            success: ip_v6.is_some(),
            ip: ip_v6,
            org: org_v6,
            region: locate_v6,
            risk_score: None,
        };

        (response_v4, response_v6)
    }
}

async fn parse_ipinfo_json(result: Response) -> (Option<IpAddr>, Option<String>, Option<String>) {
    if !result.status().is_success() {
        return (None, None, None);
    }

    let json = match result.json::<Value>().await {
        Ok(json) => json,
        Err(_) => {
            return (None, None, None);
        }
    };

    let ip = json_value_to_string(&json, "ip");
    let ip = match ip {
        None => {
            return (None, None, None);
        }
        Some(ip) => match Ipv4Addr::from_str(ip.as_str()) {
            Ok(ip) => IpAddr::V4(ip),
            Err(_) => match Ipv6Addr::from_str(ip.as_str()) {
                Ok(ip) => IpAddr::V6(ip),
                Err(_) => {
                    return (None, None, None);
                }
            },
        },
    };

    let org = json_value_to_string(&json, "org");

    let region = json_value_to_string(&json, "region");
    let country = json_value_to_string(&json, "country");
    let city = json_value_to_string(&json, "city");

    let mut locate = String::new();
    if let Some(country) = country {
        locate.push_str(country.as_str());
    }
    if region.is_some() {
        locate.push_str(format!(" - {}", region.unwrap()).as_str());
    }
    if city.is_some() {
        locate.push_str(format!(" - {}", city.unwrap()).as_str());
    }

    (Some(ip), org, Some(locate))
}
