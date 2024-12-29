// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, UA_BROWSER,
};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;

pub struct BBCIPlayer;

#[async_trait]
impl Service for BBCIPlayer {
    fn name(&self) -> String {
        "BBC iPlayer".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), true, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match get_url(
            self.name(),
            &client,
            "https://open.live.bbc.co.uk/mediaselector/6/select/version/2.0/mediaset/pc/vpid/bbc_one_london/format/json/jsfunc/JS_callbacks0",
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

        let geolocation = html.contains("geolocation");
        let vs_hls_push_uk = html.contains("vs-hls-push-uk");

        if geolocation && vs_hls_push_uk {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Page Error")),
            }
        } else if geolocation {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            }
        } else if vs_hls_push_uk {
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
                error: Some(String::from("Can not get response status code")),
            }
        }
    }
}
