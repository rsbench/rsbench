use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProviderV4, IPCheckProviderV6};
use async_trait::async_trait;
use reqwest::Response;
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct VoreAPI;

#[async_trait]
impl IPCheck for VoreAPI {
    fn provider_name(&self) -> String {
        "Vore.top".to_string()
    }

    async fn check(&self) -> (IPCheckProviderV4, IPCheckProviderV6) {
        let handle_v4 = tokio::spawn(async move {
            let Ok(client_v4) = create_reqwest_client(Some("curl/8.11.1"), false).await else {
                return (None, None);
            };

            let Ok(result) = client_v4
                .get(" https://api.vore.top/api/IPdata?ip=")
                .send()
                .await
            else {
                return (None, None);
            };

            parse_vore_json(result).await
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

async fn parse_vore_json(result: Response) -> (Option<IpAddr>, Option<String>) {
    if !result.status().is_success() {
        return (None, None);
    }

    let Ok(json) = result.json::<Value>().await else {
        return (None, None);
    };

    let ipinfo = json.get("ipinfo");
    let ip = if let Some(ipinfo) = ipinfo {
        let ip = json_value_to_string(ipinfo, "text");
        if let Some(ip) = ip {
            match Ipv4Addr::from_str(ip.as_str()) {
                Ok(ip) => IpAddr::V4(ip),
                Err(_) => match Ipv6Addr::from_str(ip.as_str()) {
                    Ok(ip) => IpAddr::V6(ip),
                    Err(_) => {
                        return (None, None);
                    }
                },
            }
        } else {
            return (None, None);
        }
    } else {
        return (None, None);
    };

    let adcode = json.get("adcode");
    let locate = if let Some(adcode) = adcode {
        json_value_to_string(adcode, "o")
    } else {
        None
    };

    (Some(ip), locate)
}
