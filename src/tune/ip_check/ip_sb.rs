use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProviderV4, IPCheckProviderV6};
use async_trait::async_trait;
use reqwest::{header, Response};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct IpSb;

#[async_trait]
impl IPCheck for IpSb {
    fn provider_name(&self) -> String {
        "Ip.sb".to_string()
    }

    async fn check(&self) -> (IPCheckProviderV4, IPCheckProviderV6) {
        let handle_v4 = tokio::spawn(async move {
            let Ok(client_v4) = create_reqwest_client(None, false).await else {
                return (None, None, None);
            };

            let Ok(result) = client_v4
                .get("https://api.ip.sb/geoip/")
                .headers(headers())
                .send()
                .await
            else {
                return (None, None, None);
            };
            parse_ipsb_json(result).await
        });

        let handle_v6 = tokio::spawn(async move {
            let Ok(client_v6) = create_reqwest_client(None, true).await else {
                return (None, None, None);
            };

            let Ok(result) = client_v6
                .get("https://api.ip.sb/geoip/")
                .headers(headers())
                .send()
                .await
            else {
                return (None, None, None);
            };
            parse_ipsb_json(result).await
        });

        let (ip_v4, org_v4, region_v4) = handle_v4.await.unwrap_or((None, None, None));
        let (ip_v6, org_v6, region_v6) = handle_v6.await.unwrap_or((None, None, None));

        let response_v4 = IPCheckProviderV4 {
            provider: self.provider_name(),
            success: ip_v4.is_some(),
            ip: ip_v4,
            org: org_v4,
            region: region_v4,
            risk_score: None,
        };

        let response_v6 = IPCheckProviderV6 {
            provider: self.provider_name(),
            success: ip_v6.is_some(),
            ip: ip_v6,
            org: org_v6,
            region: region_v6,
            risk_score: None,
        };

        (response_v4, response_v6)
    }
}

async fn parse_ipsb_json(response: Response) -> (Option<IpAddr>, Option<String>, Option<String>) {
    if !response.status().is_success() {
        return (None, None, None);
    }

    let Ok(json) = response.json::<serde_json::Value>().await else {
        return (None, None, None);
    };

    let ip = json_value_to_string(&json, "ip");

    let ip = if let Some(ip) = ip {
        match Ipv4Addr::from_str(ip.as_str()) {
            Ok(ip) => Some(IpAddr::V4(ip)),
            Err(_) => match Ipv6Addr::from_str(ip.as_str()) {
                Ok(ip) => Some(IpAddr::V6(ip)),
                Err(_) => None,
            },
        }
    } else {
        return (None, None, None);
    };

    let isp = json_value_to_string(&json, "isp");
    let org = json_value_to_string(&json, "asn_organization");
    let asn = if let Some(asn) = json.get("asn") {
        asn.as_u64()
    } else {
        None
    };
    let mut org_build = Vec::new();
    if let Some(asn) = asn {
        org_build.push(format!("AS{asn}"));
    }
    if let Some(isp) = isp {
        org_build.push(isp);
    }
    if let Some(org) = org {
        org_build.push(org);
    }
    let org_string = org_build.join(" - ");
    let org_string = if org_string.is_empty() {
        None
    } else {
        Some(org_string)
    };

    let country = json_value_to_string(&json, "country");
    let region = json_value_to_string(&json, "region");
    let city = json_value_to_string(&json, "city");
    let mut region_build = Vec::new();
    if let Some(country) = country {
        region_build.push(country);
    }
    if let Some(region) = region {
        region_build.push(region);
    }
    if let Some(city) = city {
        region_build.push(city);
    }
    let region_string = region_build.join(" - ");
    let region_string = if region_string.is_empty() {
        None
    } else {
        Some(region_string)
    };

    (ip, org_string, region_string)
}

fn headers() -> reqwest::header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
    headers.insert("cache-control", "max-age=0".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("priority", "u=0, i".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".parse().unwrap());

    headers
}
