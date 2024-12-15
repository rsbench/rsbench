// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::utils::trim_string;
use async_trait::async_trait;
use regex::Regex;
use reqwest::{header, Client};
use std::time::Duration;

pub struct YoutubePremium;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;

#[async_trait]
impl Service for YoutubePremium {
    fn name(&self) -> &'static str {
        "Youtube Premium"
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
                    service_name: "Youtube Premium".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };

        let mut headers = header::HeaderMap::new();
        headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
        headers.insert(header::COOKIE, "YSC=FSCWhKo2Zgw; VISITOR_PRIVACY_METADATA=CgJERRIEEgAgYQ%3D%3D; PREF=f7=4000; __Secure-YEC=CgtRWTBGTFExeV9Iayjele2yBjIKCgJERRIEEgAgYQ%3D%3D; SOCS=CAISOAgDEitib3FfaWRlbnRpdHlmcm9udGVuZHVpc2VydmVyXzIwMjQwNTI2LjAxX3AwGgV6aC1DTiACGgYIgMnpsgY; VISITOR_INFO1_LIVE=Di84mAIbgKY; __Secure-BUCKET=CGQ".parse().unwrap());

        let result = match client
            .get("https://www.youtube.com/premium")
            .headers(headers)
            .send()
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return UnlockResult {
                    service_name: "Youtube Premium".to_string(),
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
                    service_name: "Youtube Premium".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };

        if html.contains("www.google.cn") {
            return UnlockResult {
                service_name: "Youtube Premium".to_string(),
                available: false,
                region: Some("CN".to_string()),
                error: Some(String::from("Powered by GFW")),
            };
        }

        if html.contains("Premium is not available in your country") {
            return UnlockResult {
                service_name: "Youtube Premium".to_string(),
                available: false,
                region: None,
                error: Some(String::from("Premium is not available in your country")),
            };
        }

        let re = Regex::new(r#""INNERTUBE_CONTEXT_GL":"[A-Z]{2}"#).unwrap(); // "INNERTUBE_CONTEXT_GL":"HK"

        match re.find(&html) {
            None => UnlockResult {
                service_name: "Youtube Premium".to_string(),
                available: true,
                region: None,
                error: None,
            },
            Some(line) => {
                let region = trim_string(line.as_str(), 24, 0);
                UnlockResult {
                    service_name: "Youtube Premium".to_string(),
                    available: true,
                    region: Some(region.to_string()),
                    error: None,
                }
            }
        }
    }
}
