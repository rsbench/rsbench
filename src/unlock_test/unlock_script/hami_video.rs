// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, UA_BROWSER2,
};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;

pub struct HamiVideo;

#[async_trait]
impl Service for HamiVideo {
    fn name(&self) -> String {
        "HamiVideo".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER2), true, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match get_url(
            self.name(),
            &client,
            "https://hamivideo.hinet.net/api/play.do?id=OTT_VOD_0000249064&freeProduct=1",
            None,
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

        if html.contains("06001-107") {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
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
