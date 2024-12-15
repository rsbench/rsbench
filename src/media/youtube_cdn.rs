// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{MediaService, UnlockResult};
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use std::time::Duration;

pub struct YoutubeCDN;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;

#[async_trait]
impl MediaService for YoutubeCDN {
    fn name(&self) -> &'static str {
        "Youtube CDN"
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match Client::builder().timeout(Duration::from_secs(10)).build() {
            Ok(client) => client,
            Err(_) => {
                return UnlockResult {
                    service_name: "Youtube CDN".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };

        let result = match client
            .get("https://redirector.googlevideo.com/report_mapping")
            .send()
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return UnlockResult {
                    service_name: "Youtube CDN".to_string(),
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
                    service_name: "Youtube CDN".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };

        let re = Regex::new(r#"=>"#).unwrap();

        let mut first_line = "";
        for line in html.lines() {
            if re.is_match(line) {
                first_line = line;
                break;
            }
        }

        let binding = first_line.split_whitespace().collect::<Vec<&str>>();
        let mut cdn_node = match binding.get(2) {
            None => {
                return UnlockResult {
                    service_name: "Youtube CDN".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not get CDN node")),
                }
            }
            Some(cdn_node) => cdn_node,
        };

        let cdn_node = if cdn_node.contains("-") {
            let binding = cdn_node.split("-").collect::<Vec<&str>>();
            match binding.get(1) {
                None => {
                    return UnlockResult {
                        service_name: "Youtube CDN".to_string(),
                        available: false,
                        region: None,
                        error: Some(String::from("Can not get CDN node")),
                    }
                }
                Some(cdn_node) => cdn_node.to_uppercase(),
            }
        } else {
            cdn_node.to_uppercase()
        };

        let mut count = 0;
        let mut cdn_region = String::new();
        for char in cdn_node.chars() {
            if count == 3 {
                break;
            }
            cdn_region.push(char);
            count += 1;
        }

        UnlockResult {
            service_name: "Youtube CDN".to_string(),
            available: true,
            region: Some(cdn_region),
            error: None,
        }
    }
}
