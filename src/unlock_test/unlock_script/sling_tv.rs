// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, UA_BROWSER,
};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;

pub struct SlingTV;

#[async_trait]
impl Service for SlingTV {
    fn name(&self) -> String {
        "Sling TV".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), true, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match get_url(self.name(), &client, "https://www.sling.com/", None, None).await
        {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        let status_code = result.status().as_u16();

        let html = match parse_response_to_html(self.name(), result).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        if html.contains("SORRY") {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some("Not available".to_string()),
            }
        } else if status_code == 200 {
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
