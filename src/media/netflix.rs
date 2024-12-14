use super::{MediaService, UnlockResult};
use async_trait::async_trait;
use regex::Regex;
use reqwest::{header, Client};
use std::time::Duration;

pub struct Netflix;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;

#[async_trait]
impl MediaService for Netflix {
    async fn check_unlock(&self) -> UnlockResult {
        let client = match Client::builder().timeout(Duration::from_secs(10)).build() {
            Ok(client) => client,
            Err(_) => {
                return UnlockResult {
                    service_name: "Netflix".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };

        let mut headers = header::HeaderMap::new();
        {
            headers.insert("host", "www.netflix.com".parse().unwrap());
            headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
            headers.insert(
                "sec-ch-ua",
                r#""Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24""#
                    .parse()
                    .unwrap(),
            );
            headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
            headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
            headers.insert("sec-fetch-site", "none".parse().unwrap());
            headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
            headers.insert("sec-fetch-user", "?1".parse().unwrap());
            headers.insert("sec-fetch-dest", "document".parse().unwrap());
            headers.insert("user-agent", UA_BROWSER.parse().unwrap());
        }

        let result1 = match client
            .get("https://www.netflix.com/title/81280792")
            .headers(headers.clone())
            .send()
            .await
        {
            Ok(result1) => result1,
            Err(_) => {
                return UnlockResult {
                    service_name: "Netflix".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Not available / Network connection error")),
                }
            }
        };

        tokio::time::sleep(Duration::from_secs(1)).await;

        let result2 = match client
            .get("https://www.netflix.com/title/70143836")
            .headers(headers.clone())
            .send()
            .await
        {
            Ok(result2) => result2,
            Err(_) => {
                return UnlockResult {
                    service_name: "Netflix".to_string(),
                    available: false,
                    region: None,
                    error: Some(String::from("Not available / Network connection error")),
                }
            }
        };

        if result1.status().as_u16() == 404 && result2.status().as_u16() == 404 {
            UnlockResult {
                service_name: "Netflix".to_string(),
                available: true,
                region: Some("Original Web Series".to_string()),
                error: None,
            }
        } else if result1.status().as_u16() == 403 || result2.status().as_u16() == 403 {
            UnlockResult {
                service_name: "Netflix".to_string(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            }
        } else if result1.status().as_u16() == 200 || result2.status().as_u16() == 200 {
            let region = match client
                .get("https://www.netflix.com/")
                .headers(headers)
                .send()
                .await
            {
                Ok(region) => region,
                Err(_) => {
                    return UnlockResult {
                        service_name: "Netflix".to_string(),
                        available: false,
                        region: None,
                        error: Some(String::from("Network connection error")),
                    }
                }
            };
            let html = region.text().await.unwrap();

            let re = Regex::new(r#""id":"([A-Z]{2})""#).unwrap();
            let country = match re.find(html.as_str()) {
                None => {
                    return UnlockResult {
                        service_name: "Netflix".to_string(),
                        available: false,
                        region: None,
                        error: Some(String::from("Can not parse HTML")),
                    }
                }
                Some(country) => country.as_str(),
            };
            let re = Regex::new(r#"([A-Z]{2})"#).unwrap();
            let country = re.find(&country).unwrap().as_str();

            UnlockResult {
                service_name: "Netflix".to_string(),
                available: true,
                region: Some(country.to_string()),
                error: None,
            }
        } else {
            UnlockResult {
                service_name: "Netflix".to_string(),
                available: false,
                region: None,
                error: Some(String::from("Not available / Network connection error")),
            }
        }
    }
}
