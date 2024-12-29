// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::headers::nowe_headers;
use crate::unlock_test::utils::{create_reqwest_client, parse_response_to_html, UA_BROWSER};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;

pub struct NowE;

#[async_trait]
impl Service for NowE {
    fn name(&self) -> String {
        "Now E".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), true, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match client.post("https://webtvapi.nowe.com/16/1/getVodURL")
            .headers(nowe_headers())
            .body("{\"contentId\":\"202403181904703\",\"contentType\":\"Vod\",\"pin\":\"\",\"deviceName\":\"Browser\",\"deviceId\":\"w-663bcc51-913c-913c-913c-913c913c\",\"deviceType\":\"WEB\",\"secureCookie\":null,\"callerReferenceNo\":\"W17151951620081575\",\"profileId\":null,\"mupId\":null}")
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
