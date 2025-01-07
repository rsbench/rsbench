use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProvider};
use async_trait::async_trait;
use reqwest::Response;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct IPQueryIo;

#[async_trait]
impl IPCheck for IPQueryIo {
    fn provider_name(&self) -> String {
        "Ipquery.io".to_string()
    }

    async fn check(&self) -> IPCheckProvider {
        let handle_v4 = tokio::spawn(async move {
            let client_v4 = match create_reqwest_client(Some("curl/8.11.1"), false).await {
                Ok(client) => client,
                Err(()) => {
                    return (None, None, None, None);
                }
            };

            let result = match client_v4
                .get("https://api.ipquery.io/?format=json")
                .send()
                .await
            {
                Ok(result) => result,
                Err(_) => {
                    return (None, None, None, None);
                }
            };

            parse_ipquery_json(result).await
        });

        let handle_v6 = tokio::spawn(async move {
            let client_v4 = match create_reqwest_client(Some("curl/8.11.1"), true).await {
                Ok(client) => client,
                Err(()) => {
                    return (None, None, None, None);
                }
            };

            let result = match client_v4
                .get("https://api.ipquery.io/?format=json")
                .send()
                .await
            {
                Ok(result) => result,
                Err(_) => {
                    return (None, None, None, None);
                }
            };

            parse_ipquery_json(result).await
        });

        let (ip_v4, org_v4, locate_v4, risk_v4) =
            handle_v4.await.unwrap_or((None, None, None, None));
        let (ip_v6, org_v6, locate_v6, risk_v6) =
            handle_v6.await.unwrap_or((None, None, None, None));

        let mut response = IPCheckProvider {
            provider: self.provider_name(),
            success: true,
            ipv4: ip_v4,
            ipv4_org: org_v4,
            ipv4_region: locate_v4,
            ipv6: ip_v6,
            ipv6_org: org_v6,
            ipv6_region: locate_v6,
            risk_score_v4: risk_v4,
            risk_score_v6: risk_v6,
        };

        if ip_v4.is_none() && ip_v6.is_none() {
            response.success = false;
        }

        response
    }
}

async fn parse_ipquery_json(
    result: Response,
) -> (Option<IpAddr>, Option<String>, Option<String>, Option<u8>) {
    let json = match result.json::<serde_json::Value>().await {
        Ok(json) => json,
        Err(_) => {
            return (None, None, None, None);
        }
    };

    let ip = json_value_to_string(&json, "ip");
    let ip = match ip {
        None => {
            return (None, None, None, None);
        }
        Some(ip) => match Ipv4Addr::from_str(ip.as_str()) {
            Ok(ip) => IpAddr::V4(ip),
            Err(_) => match Ipv6Addr::from_str(ip.as_str()) {
                Ok(ip) => IpAddr::V6(ip),
                Err(_) => {
                    return (None, None, None, None);
                }
            },
        },
    };

    let isp = json.get("isp");
    let location = json.get("location");
    let risk = json.get("risk");

    let risk_score = if risk.is_some() {
        match risk.unwrap().get("risk_score") {
            Some(risk) => match risk.as_u64() {
                Some(risk) => Some(risk as u8),
                None => None,
            },
            None => None,
        }
    } else {
        None
    };

    let (asn, org, isp) = {
        if isp.is_some() {
            let asn = json_value_to_string(isp.unwrap(), "asn");
            let org = json_value_to_string(isp.unwrap(), "org");
            let isp = json_value_to_string(isp.unwrap(), "isp");
            (asn, org, isp)
        } else {
            (None, None, None)
        }
    };

    let (country, state, city) = {
        if location.is_some() {
            let country = json_value_to_string(location.unwrap(), "country");
            let state = json_value_to_string(location.unwrap(), "state");
            let city = json_value_to_string(location.unwrap(), "city");
            (country, state, city)
        } else {
            (None, None, None)
        }
    };

    let mut locate = String::new();
    if country.is_some() {
        locate.push_str(country.unwrap().as_str());
    }
    if state.is_some() {
        locate.push_str(" - ");
        locate.push_str(state.unwrap().as_str());
    }
    if city.is_some() {
        locate.push_str(" - ");
        locate.push_str(city.unwrap().as_str());
    }
    let locate = if locate.is_empty() {
        None
    } else {
        Some(locate)
    };

    let mut isp_string = String::new();
    if asn.is_some() {
        isp_string.push_str(asn.unwrap().as_str());
    }
    if org.is_some() {
        isp_string.push_str(" - ");
        isp_string.push_str(org.unwrap().as_str());
    }
    if isp.is_some() {
        isp_string.push_str(" - ");
        isp_string.push_str(isp.unwrap().as_str());
    }
    let isp_string = if isp_string.is_empty() {
        None
    } else {
        Some(isp_string)
    };

    (Some(ip), isp_string, locate, risk_score)
}
