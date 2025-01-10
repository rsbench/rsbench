use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProviderV4, IPCheckProviderV6};
use async_trait::async_trait;
use reqwest::Response;
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct PcOnline;

#[async_trait]
impl IPCheck for PcOnline {
    fn provider_name(&self) -> String {
        "PcOnline".to_string()
    }

    async fn check(&self) -> (IPCheckProviderV4, IPCheckProviderV6) {
        let handle_v4 = tokio::spawn(async move {
            let client_v4 = match create_reqwest_client(Some("curl/8.11.1"), false).await {
                Ok(client) => client,
                Err(()) => {
                    return (None, None);
                }
            };

            let result = match client_v4.get("https://myip.ipip.net/json").send().await {
                Ok(result) => result,
                Err(_) => {
                    return (None, None);
                }
            };

            parse_pconline_json(result).await
        });

        let (ip_v4, locate_v4) = handle_v4.await.unwrap_or((None, None));

        let response_v4 = IPCheckProviderV4 {
            provider: self.provider_name(),
            success: ip_v4.is_some(),
            ip: ip_v4,
            org: locate_v4,
            region: None,
            risk_score: None,
        };

        let response_v6 = IPCheckProviderV6 {
            provider: self.provider_name(),
            success: false,
            ip: None,
            org: None,
            region: None,
            risk_score: None,
        };

        (response_v4, response_v6)
    }
}

async fn parse_pconline_json(result: Response) -> (Option<IpAddr>, Option<String>) {
    if !result.status().is_success() {
        return (None, None);
    }

    let json = match result.json::<Value>().await {
        Ok(json) => json,
        Err(_) => {
            return (None, None);
        }
    };

    if json_value_to_string(&json, "ret") != Some("ok".to_string()) {
        return (None, None);
    }

    let data = match json.get("data") {
        Some(data) => data,
        None => {
            return (None, None);
        }
    };

    let ip = json_value_to_string(data, "ip");
    let ip = match ip {
        None => {
            return (None, None);
        }
        Some(ip) => match Ipv4Addr::from_str(ip.as_str()) {
            Ok(ip) => IpAddr::V4(ip),
            Err(_) => match Ipv6Addr::from_str(ip.as_str()) {
                Ok(ip) => IpAddr::V6(ip),
                Err(_) => {
                    return (None, None);
                }
            },
        },
    };

    let locate = json_value_to_string(data, "addr");

    (Some(ip), locate)
}
