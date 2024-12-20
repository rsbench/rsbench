// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::headers::default_headers;
use crate::unlock_test::utils::{create_reqwest_client, get_url, parse_response_to_html};
use async_trait::async_trait;
use regex::Regex;

pub struct Steam;

#[async_trait]
impl Service for Steam {
    fn name(&self) -> String {
        "Steam".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client =
            match create_reqwest_client(self.name(), Some(super::utils::UA_BROWSER), false, None)
                .await
            {
                Ok(client) => client,
                Err(unlock_result) => return unlock_result,
            };

        let result = match get_url(
            self.name(),
            &client,
            "https://store.steampowered.com/app/761830",
            Some(default_headers()),
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

        let re = Regex::new("priceCurrency").unwrap();

        let mut region = String::new();
        for line in html.lines() {
            if re.is_match(line) {
                region = crate::unlock_test::utils::trim_string(line.trim(), 40, 2).to_uppercase();
            }
        }

        UnlockResult {
            service_name: self.name(),
            available: true,
            region: Some(region.to_string()),
            error: None,
        }
    }
}
