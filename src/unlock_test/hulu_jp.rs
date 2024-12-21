// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::headers::hulu_jp_headers;
use crate::unlock_test::utils::{create_reqwest_client, get_url, parse_response_to_html};
use async_trait::async_trait;

pub struct HuluJP;

#[async_trait]
impl Service for HuluJP {
    fn name(&self) -> String {
        "Hulu Japan".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client =
            match create_reqwest_client(self.name(), Some(super::utils::UA_BROWSER), true, None)
                .await
            {
                Ok(client) => client,
                Err(unlock_result) => return unlock_result,
            };

        let result = match get_url(
            self.name(),
            &client,
            "https://id.hulu.jp/",
            Some(hulu_jp_headers()),
            None,
        )
        .await
        {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        let status_code = result.status().as_u16();

        let html = match parse_response_to_html(self.name(), result).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        if html.contains("restrict") {
            return UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            };
        }

        if status_code == 200 {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
            }
        } else if status_code == 403 {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            }
        } else {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Unknown error")),
            }
        }
    }
}
