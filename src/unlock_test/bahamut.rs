// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::utils::trim_string;
use async_trait::async_trait;
use regex::Regex;
use reqwest::{header, Client};
use std::time::Duration;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;
pub struct BahamutAnime;

#[async_trait]
impl Service for BahamutAnime {
    fn name(&self) -> &'static str {
        "Bahamut Anime"
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match Client::builder()
            .timeout(Duration::from_secs(15))
            .cookie_store(true)
            .user_agent(UA_BROWSER)
            .build()
        {
            Ok(client) => client,
            Err(_) => {
                return UnlockResult {
                    service_name: "Bahamut Anime".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };

        let result_device_id = match client
            .get("https://ani.gamer.com.tw/ajax/getdeviceid.php")
            .send()
            .await
        {
            Ok(result_device_id) => result_device_id,
            Err(_) => {
                return UnlockResult {
                    service_name: "Bahamut Anime".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Not available / Network connection error")),
                }
            }
        };
        let html = match result_device_id.text().await {
            Ok(html) => html,
            Err(_) => {
                return UnlockResult {
                    service_name: "Bahamut Anime".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };
        let device_id = trim_string(&html, 13, 2);

        match client
            .get(format!(
                "https://ani.gamer.com.tw/ajax/token.php?adID=89422&sn=37783&device={}",
                device_id
            ))
            .send()
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return UnlockResult {
                    service_name: "Bahamut Anime".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Not available / Network connection error")),
                }
            }
        };

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "accept",
            "*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
                .parse()
                .unwrap(),
        );
        headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
        headers.insert(
            "sec-ch-ua",
            r#""Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24""#
                .parse()
                .unwrap(),
        );
        headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
        headers.insert("sec-ch-ua-model", "\"\"".parse().unwrap());
        headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
        headers.insert("sec-ch-ua-platform-version", "\"15.0.0\"".parse().unwrap());
        headers.insert("sec-fetch-dest", "document".parse().unwrap());
        headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
        headers.insert("sec-fetch-site", "none".parse().unwrap());
        headers.insert("sec-fetch-user", "?1".parse().unwrap());

        let result2 = match client
            .get("https://ani.gamer.com.tw/")
            .headers(headers)
            .send()
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return UnlockResult {
                    service_name: "Bahamut Anime".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Not available / Network connection error")),
                }
            }
        };

        let html2 = match result2.text().await {
            Ok(html) => html,
            Err(_) => {
                return UnlockResult {
                    service_name: "Bahamut Anime".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };

        let re = Regex::new(r#"data-geo="([A-Za-z]{2})""#).unwrap();

        let mut region = String::new();

        for line in re.find_iter(&html2) {
            region = trim_string(&line.as_str(), 10, 1)
                .to_string()
                .to_uppercase();
        }

        UnlockResult {
            service_name: "Bahamut Anime".to_string(),
            available: true,
            region: Some(region),
            error: None,
        }
    }
}
