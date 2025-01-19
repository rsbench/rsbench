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
            let Ok(client_v4) = create_reqwest_client(Some("curl/8.11.1"), false).await else {
                return None;
            };

            let Ok(result) = client_v4
                .get("https://whois.pconline.com.cn/ipJson.jsp?ip=&json=true")
                .send()
                .await
            else {
                return None;
            };

            parse_pconline_json(result).await
        });

        let ip_v4 = handle_v4.await.unwrap_or(None);

        let response_v4 = IPCheckProviderV4 {
            provider: self.provider_name(),
            success: ip_v4.is_some(),
            ip: ip_v4,
            org: None,
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

async fn parse_pconline_json(result: Response) -> Option<IpAddr> {
    if !result.status().is_success() {
        return None;
    }

    let Ok(text) = result.text().await else {
        return None;
    };

    let text = text.trim();

    let Ok(json) = serde_json::from_str::<Value>(text) else {
        return None;
    };

    let ip = json_value_to_string(&json, "ip");
    let ip = match ip {
        None => {
            return None;
        }
        Some(ip) => match Ipv4Addr::from_str(ip.as_str()) {
            Ok(ip) => IpAddr::V4(ip),
            Err(_) => match Ipv6Addr::from_str(ip.as_str()) {
                Ok(ip) => IpAddr::V6(ip),
                Err(_) => {
                    return None;
                }
            },
        },
    };

    Some(ip)
}
