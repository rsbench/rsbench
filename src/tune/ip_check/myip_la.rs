use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProvider};
use async_trait::async_trait;
use reqwest::Response;
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct MyIpLa;

#[async_trait]
impl IPCheck for MyIpLa {
    fn provider_name(&self) -> String {
        "Myip.la".to_string()
    }

    async fn check(&self) -> IPCheckProvider {
        let handle_v4 = tokio::spawn(async move {
            let client_v4 = match create_reqwest_client(Some("curl/8.11.1"), false).await {
                Ok(client) => client,
                Err(()) => {
                    return (None, None);
                }
            };

            let result = match client_v4.get("https://api.myip.la/cn?json").send().await {
                Ok(result) => result,
                Err(_) => {
                    return (None, None);
                }
            };

            parse_myipla_json(result).await
        });

        let handle_v6 = tokio::spawn(async move {
            let client_v6 = match create_reqwest_client(Some("curl/8.11.1"), true).await {
                Ok(client) => client,
                Err(()) => {
                    return (None, None);
                }
            };

            let result = match client_v6.get("https://api.myip.la/cn?json").send().await {
                Ok(result) => result,
                Err(_) => {
                    return (None, None);
                }
            };

            parse_myipla_json(result).await
        });

        let (ip_v4, locate_v4) = handle_v4.await.unwrap_or((None, None));
        let (ip_v6, locate_v6) = handle_v6.await.unwrap_or((None, None));

        let mut response = IPCheckProvider {
            provider: self.provider_name(),
            success: true,
            ipv4: ip_v4,
            ipv4_org: None,
            ipv4_region: locate_v4,
            ipv6: ip_v6,
            ipv6_org: None,
            ipv6_region: locate_v6,
            risk_score_v4: None,
            risk_score_v6: None,
        };

        if ip_v4.is_none() && ip_v6.is_none() {
            response.success = false;
        }

        response
    }
}

async fn parse_myipla_json(result: Response) -> (Option<IpAddr>, Option<String>) {
    if !result.status().is_success() {
        return (None, None);
    }

    let json = match result.json::<Value>().await {
        Ok(json) => json,
        Err(_) => {
            return (None, None);
        }
    };

    let ip = json_value_to_string(&json, "ip");
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

    let location = match json.get("location") {
        None => {
            return (None, None);
        }
        Some(loc) => loc,
    };

    let country_name = json_value_to_string(location, "country_name");
    let province = json_value_to_string(location, "province");
    let city = json_value_to_string(location, "city");

    let mut loc = String::new();
    if let Some(country_name) = country_name {
        loc.push_str(&country_name);
    }
    if let Some(province) = province {
        loc.push_str(" - ");
        loc.push_str(&province);
    }
    if let Some(city) = city {
        loc.push_str(" - ");
        loc.push_str(&city);
    }

    (Some(ip), Some(loc))
}
