// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::headers::netflix_headers;
use crate::unlock_test::utils::{create_reqwest_client, get_url, parse_response_to_html};
use async_trait::async_trait;
use regex::Regex;
use std::time::Duration;

pub struct Netflix;

#[async_trait]
impl Service for Netflix {
    fn name(&self) -> String {
        "Netflix".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client =
            match create_reqwest_client(self.name(), Some(super::utils::UA_BROWSER), false, None)
                .await
            {
                Ok(client) => client,
                Err(unlock_result) => return unlock_result,
            };

        let result1 = match get_url(
            self.name(),
            &client,
            "https://www.netflix.com/title/81280792",
            Some(netflix_headers()),
        )
        .await
        {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        tokio::time::sleep(Duration::from_millis(100)).await;

        let result2 = match get_url(
            self.name(),
            &client,
            "https://www.netflix.com/title/70143836",
            Some(netflix_headers()),
        )
        .await
        {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        if result1.status().as_u16() == 404 && result2.status().as_u16() == 404 {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: Some("Original Web Series".to_string()),
                error: None,
            }
        } else if result1.status().as_u16() == 403 || result2.status().as_u16() == 403 {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            }
        } else if result1.status().as_u16() == 200 || result2.status().as_u16() == 200 {
            let region = match get_url(
                self.name(),
                &client,
                "https://www.netflix.com/",
                Some(netflix_headers()),
            )
            .await
            {
                Ok(result) => result,
                Err(unlock_result) => return unlock_result,
            };

            let html = match parse_response_to_html(self.name(), region).await {
                Ok(html) => html,
                Err(unlock_result) => return unlock_result,
            };

            let re = Regex::new(r#""id":"([A-Z]{2})""#).unwrap();
            let country = match re.find(html.as_str()) {
                None => {
                    return UnlockResult {
                        service_name: self.name(),
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
                service_name: self.name(),
                available: true,
                region: Some(country.to_string()),
                error: None,
            }
        } else {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available / Network connection error")),
            }
        }
    }
}
