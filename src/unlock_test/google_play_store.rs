// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use async_trait::async_trait;
use regex::Regex;
use reqwest::{header, Client};
use std::time::Duration;

pub struct GooglePlayStore;

#[async_trait]
impl Service for GooglePlayStore {
    fn name(&self) -> String {
        "Google Play Store".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match Client::builder().timeout(Duration::from_secs(5)).build() {
            Ok(client) => client,
            Err(_) => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };

        let mut headers = header::HeaderMap::new();
        headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
        headers.insert("accept-language", "en-US;q=0.9".parse().unwrap());
        headers.insert("priority", "u=0, i".parse().unwrap());
        headers.insert(
            "sec-ch-ua",
            "\"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\", \"Google Chrome\";v=\"131\""
                .parse()
                .unwrap(),
        );
        headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
        headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
        headers.insert("sec-fetch-dest", "document".parse().unwrap());
        headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
        headers.insert("sec-fetch-site", "none".parse().unwrap());
        headers.insert("sec-fetch-user", "?1".parse().unwrap());
        headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
        headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".parse().unwrap());

        let result = match client
            .get("https://play.google.com/")
            .headers(headers)
            .send()
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Not available / Network connection error")),
                }
            }
        };

        let html = match result.text().await {
            Ok(html) => html,
            Err(_) => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };

        let re = Regex::new(r#"<div class="yVZQTb">\s*([^<(]+)"#).unwrap();
        let region = match re.captures(&html) {
            Some(captures) => {
                let region = match captures.get(1) {
                    None => {
                        return UnlockResult {
                            service_name: self.name(),
                            available: false,
                            region: None,
                            error: Some(String::from("Can not get country code")),
                        }
                    }
                    Some(region) => region.as_str().trim(),
                };
                Some(region.to_string())
            }
            None => None,
        };

        UnlockResult {
            service_name: self.name(),
            available: true,
            region,
            error: None,
        }
    }
}
