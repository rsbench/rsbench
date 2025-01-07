use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProvider};
use async_trait::async_trait;
use reqwest::Response;
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct IpIpNet;

#[async_trait]
impl IPCheck for IpIpNet {
    fn provider_name(&self) -> String {
        "Ipip.net".to_string()
    }

    async fn check(&self) -> IPCheckProvider {
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

            parse_ipipnet_json(result).await
        });

        let (ip_v4, locate_v4) = handle_v4.await.unwrap_or((None, None));

        let mut response = IPCheckProvider {
            provider: self.provider_name(),
            success: true,
            ipv4: ip_v4,
            ipv4_org: None,
            ipv4_region: locate_v4,
            ipv6: None,
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

async fn parse_ipipnet_json(result: Response) -> (Option<IpAddr>, Option<String>) {
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

    let locations = match data.get("location") {
        None => {
            return (None, None);
        }
        Some(loc) => loc,
    };

    let locations = match locations.as_array() {
        None => {
            return (None, None);
        }
        Some(loc) => loc,
    };

    let mut locations_str = Vec::new();
    for location in locations {
        let location = match location.as_str() {
            None => {
                return (None, None);
            }
            Some(location) => location,
        };
        if location.is_empty() {
            continue;
        }
        locations_str.push(location.to_string());
    }

    let mut loc = String::new();
    let locations_str_len = locations_str.len();
    let mut count = 0;
    for location in locations_str {
        count += 1;
        loc.push_str(&location);
        if count < locations_str_len - 1 {
            loc.push_str(" - ");
        }
    }

    (Some(ip), Some(loc))
}
