// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, trim_string, UA_BROWSER,
};
use async_trait::async_trait;
use regex::Regex;

pub struct IqiyiOversea;

#[async_trait]
impl Service for IqiyiOversea {
    fn name(&self) -> String {
        "IQIYI Oversea".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), false, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match get_url(self.name(), &client, "https://www.iq.com/", None, None).await {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        let html = match parse_response_to_html(self.name(), result).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        let re = Regex::new(r#"mod=[a-z]+"#).unwrap();
        let region = match re.find(&html) {
            None => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not get country code")),
                }
            }
            Some(region) => region.as_str(),
        };

        let region = trim_string(region, 4, 0).to_uppercase();

        UnlockResult {
            service_name: self.name(),
            available: true,
            region: Some(region),
            error: None,
        }
    }
}
