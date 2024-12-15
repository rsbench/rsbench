// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use std::time::Duration;

pub struct Steam;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;

#[async_trait]
impl Service for Steam {
    fn name(&self) -> &'static str {
        "Steam"
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match Client::builder().timeout(Duration::from_secs(10)).build() {
            Ok(client) => client,
            Err(_) => {
                return UnlockResult {
                    service_name: "Steam".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };

        let result = match client
            .get("https://store.steampowered.com/app/761830")
            .send()
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return UnlockResult {
                    service_name: "Steam".to_string(),
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
                    service_name: "Steam".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };

        let re = Regex::new("priceCurrency").unwrap();

        let mut region = String::new();
        for line in html.lines() {
            if re.is_match(line) {
                region = crate::unlock_test::utils::trim_string(line.trim(), 40, 2).to_uppercase();
            }
        }

        UnlockResult {
            service_name: "Steam".to_string(),
            available: true,
            region: Some(region.to_string()),
            error: None,
        }
    }
}
