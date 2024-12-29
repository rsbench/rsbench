// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::headers::google_play_store_headers;
use crate::unlock_test::utils::{create_reqwest_client, get_url, parse_response_to_html};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;
use regex::Regex;

pub struct GooglePlayStore;

#[async_trait]
impl Service for GooglePlayStore {
    fn name(&self) -> String {
        "Google Play Store".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), None, false, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match get_url(
            self.name(),
            &client,
            "https://play.google.com/",
            Some(google_play_store_headers()),
            None,
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
