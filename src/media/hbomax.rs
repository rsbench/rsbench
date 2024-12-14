// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{MediaService, UnlockResult};
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use std::time::Duration;

pub struct HboMax;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;

#[async_trait]
impl MediaService for HboMax {
    async fn check_unlock(&self) -> UnlockResult {
        let client = match Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent(UA_BROWSER)
            .build()
        {
            Ok(client) => client,
            Err(_) => {
                return UnlockResult {
                    service_name: "HBO MAX".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };

        let result = match client.get("https://www.max.com/").send().await {
            Ok(result1) => result1,
            Err(_) => {
                return UnlockResult {
                    service_name: "HBO MAX".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Not available / Network connection error")),
                }
            }
        };

        if result.status().as_u16() != 200 {
            return UnlockResult {
                service_name: "HBO MAX".to_string(),
                available: false,
                region: None,
                error: Some("Not available".to_string()),
            };
        }

        let html = match result.text().await {
            Ok(html) => html,
            Err(_) => {
                return UnlockResult {
                    service_name: "HBO MAX".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };

        let mut country_list = Vec::new();
        let re = Regex::new(r#""url":"/[a-z]{2}/[a-z]{2}""#).unwrap(); // "url":"/tr/en"

        for line in re.find_iter(&html) {
            country_list.push(trim_string(line.as_str(), 8, 4).to_uppercase());
        }

        let mut unique_vec: Vec<_> = country_list.iter().collect();
        unique_vec.sort();
        unique_vec.dedup();

        let country_list = unique_vec;

        let re = Regex::new(r#""countryCode":"[a-z]{2}""#).unwrap();

        let region_country_code = match re.find(&html) {
            None => {
                return UnlockResult {
                    service_name: "HBO MAX".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
            Some(country) => country.as_str(),
        };

        let region = trim_string(region_country_code, 15, 1).to_uppercase();

        if country_list.contains(&&region) {
            UnlockResult {
                service_name: "HBO MAX".to_string(),
                available: true,
                region: Some(region),
                error: None,
            }
        } else {
            UnlockResult {
                service_name: "HBO MAX".to_string(),
                available: false,
                region: Some(region),
                error: Some(std::string::String::from(
                    "Not available / Network connection error",
                )),
            }
        }
    }
}

fn trim_string(s: &str, leading: usize, trailing: usize) -> &str {
    let start = leading;
    let end = s.len().saturating_sub(trailing);
    &s[start..end]
}
