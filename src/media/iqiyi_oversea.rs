// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{MediaService, UnlockResult};
use crate::media::utils::trim_string;
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use std::time::Duration;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;
pub struct IqiyiOversea;

#[async_trait]
impl MediaService for IqiyiOversea {
    fn name(&self) -> &'static str {
        "IQIYI Oversea"
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match Client::builder()
            .timeout(Duration::from_secs(5))
            .user_agent(UA_BROWSER)
            .build()
        {
            Ok(client) => client,
            Err(_) => {
                return UnlockResult {
                    service_name: "IQIYI Oversea".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };

        let result = match client.get("https://www.iq.com/").send().await {
            Ok(result) => result,
            Err(_) => {
                return UnlockResult {
                    service_name: "IQIYI Oversea".to_string(),
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
                    service_name: "IQIYI Oversea".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };

        let re = Regex::new(r#"mod=[a-z]+"#).unwrap();
        let region = match re.find(&html) {
            None => {
                return UnlockResult {
                    service_name: "IQIYI Oversea".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not get country code")),
                }
            }
            Some(region) => region.as_str(),
        };

        let region = trim_string(region, 4, 0).to_uppercase();

        UnlockResult {
            service_name: "IQIYI Oversea".to_string(),
            available: true,
            region: Some(region),
            error: None,
        }
    }
}
