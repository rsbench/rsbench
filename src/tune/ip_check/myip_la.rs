use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProviderV4, IPCheckProviderV6};
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

    async fn check(&self) -> (IPCheckProviderV4, IPCheckProviderV6) {
        let handle_v4 = tokio::spawn(async move {
            let Ok(client_v4) = create_reqwest_client(Some("curl/8.11.1"), false).await else {
                return (None, None);
            };

            let Ok(result) = client_v4.get("https://api.myip.la/cn?json").send().await else {
                return (None, None);
            };

            parse_myipla_json(result).await
        });

        let handle_v6 = tokio::spawn(async move {
            let Ok(client_v6) = create_reqwest_client(Some("curl/8.11.1"), true).await else {
                return (None, None);
            };

            let Ok(result) = client_v6.get("https://api.myip.la/cn?json").send().await else {
                return (None, None);
            };

            parse_myipla_json(result).await
        });

        let (ip_v4, locate_v4) = handle_v4.await.unwrap_or((None, None));
        let (ip_v6, locate_v6) = handle_v6.await.unwrap_or((None, None));

        let response_v4 = IPCheckProviderV4 {
            provider: self.provider_name(),
            success: ip_v4.is_some(),
            ip: ip_v4,
            org: None,
            region: locate_v4,
            risk_score: None,
        };

        let response_v6 = IPCheckProviderV6 {
            provider: self.provider_name(),
            success: ip_v6.is_some(),
            ip: ip_v6,
            org: None,
            region: locate_v6,
            risk_score: None,
        };

        (response_v4, response_v6)
    }
}

async fn parse_myipla_json(result: Response) -> (Option<IpAddr>, Option<String>) {
    if !result.status().is_success() {
        return (None, None);
    }

    let Ok(json) = result.json::<Value>().await else {
        return (None, None);
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
