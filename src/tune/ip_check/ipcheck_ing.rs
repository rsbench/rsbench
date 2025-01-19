use crate::tune::ip_check::utils::{create_reqwest_client, json_value_to_string};
use crate::tune::ip_check::{IPCheck, IPCheckProviderV4, IPCheckProviderV6};
use async_trait::async_trait;
use reqwest::{header, Client, Response};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct IPCheckIng;

#[async_trait]
impl IPCheck for IPCheckIng {
    fn provider_name(&self) -> String {
        "Ipcheck.ing".to_string()
    }

    async fn check(&self) -> (IPCheckProviderV4, IPCheckProviderV6) {
        let handle_v4 = tokio::spawn(async move {
            let Ok(client_v4) = create_reqwest_client(Some("curl/8.11.1"), false).await else {
                return (None, None, None, None);
            };

            let Ok(result) = client_v4.get("https://4.ipcheck.ing/").send().await else {
                return (None, None, None, None);
            };

            let ip = parse_ipcheck_ing(result).await;

            if let Some(ip) = ip {
                let (risk, org, region) = get_ipcheck_ing_info(ip).await;
                (Some(ip), risk, org, region)
            } else {
                (None, None, None, None)
            }
        });

        let handle_v6 = tokio::spawn(async move {
            let Ok(client_v6) = create_reqwest_client(Some("curl/8.11.1"), true).await else {
                return (None, None, None, None);
            };

            let Ok(result) = client_v6.get("https://6.ipcheck.ing/").send().await else {
                return (None, None, None, None);
            };
            let ip = parse_ipcheck_ing(result).await;
            if let Some(ip) = ip {
                let (risk, org, region) = get_ipcheck_ing_info(ip).await;
                (Some(ip), risk, org, region)
            } else {
                (None, None, None, None)
            }
        });

        let (ip_v4, ipv4_risk, ipv4_org, ipv4_region) =
            handle_v4.await.unwrap_or((None, None, None, None));
        let (ip_v6, ipv6_risk, ipv6_org, ipv6_region) =
            handle_v6.await.unwrap_or((None, None, None, None));

        let response_v4 = IPCheckProviderV4 {
            provider: self.provider_name(),
            success: ip_v4.is_some(),
            ip: ip_v4,
            org: ipv4_org,
            region: ipv4_region,
            risk_score: ipv4_risk,
        };

        let response_v6 = IPCheckProviderV6 {
            provider: self.provider_name(),
            success: ip_v6.is_some(),
            ip: ip_v6,
            org: ipv6_org,
            region: ipv6_region,
            risk_score: ipv6_risk,
        };

        (response_v4, response_v6)
    }
}

async fn parse_ipcheck_ing(response: Response) -> Option<IpAddr> {
    if !response.status().is_success() {
        return None;
    }

    let html = match response.text().await {
        Ok(html) => html.trim().to_string(),
        Err(_) => {
            return None;
        }
    };

    let ip = match Ipv4Addr::from_str(html.as_str()) {
        Ok(ip) => IpAddr::V4(ip),
        Err(_) => match Ipv6Addr::from_str(html.as_str()) {
            Ok(ip) => IpAddr::V6(ip),
            Err(_) => {
                return None;
            }
        },
    };

    Some(ip)
}

async fn get_ipcheck_ing_info(ip: IpAddr) -> (Option<u8>, Option<String>, Option<String>) {
    let client = Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert(
        "if-none-match",
        "W/\"14c-D7tiLlSLmcGbAYEFPfRwpzWcI\"".parse().unwrap(),
    );
    headers.insert("priority", "u=1, i".parse().unwrap());
    headers.insert("referer", "https://ipcheck.ing/".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".parse().unwrap());

    let Ok(res) = client
        .get(format!(
            "https://ipcheck.ing/api/ipchecking?ip={ip}&lang=zh-CN"
        ))
        .headers(headers)
        .send()
        .await
    else {
        return (None, None, None);
    };

    let Ok(json) = res.json::<serde_json::Value>().await else {
        return (None, None, None);
    };

    let asn = json_value_to_string(&json, "asn");
    let org = json_value_to_string(&json, "org");
    let mut org_string = String::new();
    if let Some(asn) = asn {
        org_string.push_str(asn.as_str());
        org_string.push(' ');
    }
    if let Some(org) = org {
        org_string.push_str(org.as_str());
    }
    let org_string = org_string.trim();
    let org_string = if org_string.is_empty() {
        None
    } else {
        Some(org_string.to_string())
    };

    let country_name = json_value_to_string(&json, "country_name");
    let region = json_value_to_string(&json, "region");
    let city = json_value_to_string(&json, "city");
    let mut region_string = String::new();
    if let Some(country_name) = country_name {
        region_string.push_str(country_name.as_str());
    }
    if let Some(region) = region {
        if !region_string.is_empty() {
            region_string.push_str(" - ");
        }
        region_string.push_str(region.as_str());
    }
    if let Some(city) = city {
        if !region_string.is_empty() {
            region_string.push_str(" - ");
        }
        region_string.push_str(city.as_str());
    }
    let region_string = region_string.trim();
    let region_string = if region_string.is_empty() {
        None
    } else {
        Some(region_string.to_string())
    };

    let proxy = json.get("proxyDetect");
    let risk = if let Some(proxy) = proxy {
        let risk = proxy.get("risk");
        if let Some(risk) = risk {
            let risk = risk.as_u64();
            risk.map(|risk| u8::try_from(risk).unwrap_or(0))
        } else {
            None
        }
    } else {
        None
    };

    (risk, org_string, region_string)
}
