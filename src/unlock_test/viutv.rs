// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::headers::viutv_headers;
use crate::unlock_test::utils::{create_reqwest_client, parse_response_to_html};
use async_trait::async_trait;

pub struct ViuTV;

#[async_trait]
impl Service for ViuTV {
    fn name(&self) -> String {
        "ViuTV".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client =
            match create_reqwest_client(self.name(), Some(super::utils::UA_BROWSER), true, None)
                .await
            {
                Ok(client) => client,
                Err(unlock_result) => return unlock_result,
            };

        let result = match client.post("https://api.viu.now.com/p8/3/getLiveURL")
            .headers(viutv_headers())
            .body("{\"callerReferenceNo\":\"20210726112323\",\"contentId\":\"099\",\"contentType\":\"Channel\",\"channelno\":\"099\",\"mode\":\"prod\",\"deviceId\":\"29b3cb117a635d5b56\",\"deviceType\":\"ANDROID_WEB\"}")
            .send().await {
            Ok(result) => result,
            Err(_) => return UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some("Not available / Network connection error".to_string()),
            },
        };

        let html = match parse_response_to_html(self.name(), result).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        if html.contains(r#""responseCode":"SUCCESS""#) {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
            }
        } else if html.contains(r#""responseCode":"GEO_CHECK_FAIL""#) {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some("Not available".to_string()),
            }
        } else {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some("Not available".to_string()),
            }
        }
    }
}
