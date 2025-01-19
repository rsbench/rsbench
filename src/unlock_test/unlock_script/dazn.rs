// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::headers::dazn_headers;
use crate::unlock_test::utils::{
    create_reqwest_client, parse_response_to_html, trim_string, UA_BROWSER,
};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;
use regex::Regex;

pub struct Dazn;

#[async_trait]
impl Service for Dazn {
    fn name(&self) -> String {
        "Dazn".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), true, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let Ok(res) = client.post("https://startup.core.indazn.com/misl/v5/Startup")
            .headers(dazn_headers())
            .body("{\"LandingPageKey\":\"generic\",\"languages\":\"en-US,en\",\"Platform\":\"web\",\"PlatformAttributes\":{},\"Manufacturer\":\"\",\"PromoCode\":\"\",\"Version\":\"2\"}")
            .send().await else {
            return UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available / Network connection error")),
            }
        };

        let html = match parse_response_to_html(self.name(), res).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        if html.contains(r#""isAllowed":true"#) {
            let re = Regex::new(r#""GeolocatedCountry":"([a-z]{2})""#).unwrap();
            match re.find(&html) {
                None => UnlockResult {
                    service_name: self.name(),
                    available: true,
                    region: None,
                    error: None,
                },
                Some(region) => UnlockResult {
                    service_name: self.name(),
                    available: true,
                    region: Some(trim_string(region.as_str(), 21, 1).to_uppercase()),
                    error: None,
                },
            }
        } else {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            }
        }
    }
}
