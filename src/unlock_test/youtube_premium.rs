// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::headers::youtube_premium_headers;
use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, trim_string, UA_BROWSER,
};
use async_trait::async_trait;
use regex::Regex;

pub struct YoutubePremium;

#[async_trait]
impl Service for YoutubePremium {
    fn name(&self) -> String {
        "Youtube Premium".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), false, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match get_url(
            self.name(),
            &client,
            "https://www.youtube.com/premium",
            Some(youtube_premium_headers()),
        )
        .await
        {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        let html = match parse_response_to_html(self.name(), result).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        if html.contains("www.google.cn") {
            return UnlockResult {
                service_name: self.name(),
                available: false,
                region: Some("CN".to_string()),
                error: Some(String::from("Powered by GFW")),
            };
        }

        if html.contains("Premium is not available in your country") {
            return UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Premium is not available in your country")),
            };
        }

        let re = Regex::new(r#""INNERTUBE_CONTEXT_GL":"[A-Z]{2}"#).unwrap(); // "INNERTUBE_CONTEXT_GL":"HK"

        match re.find(&html) {
            None => UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
            },
            Some(line) => {
                let region = trim_string(line.as_str(), 24, 0);
                UnlockResult {
                    service_name: self.name(),
                    available: true,
                    region: Some(region.to_string()),
                    error: None,
                }
            }
        }
    }
}
