// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::headers::four_gtv_headers;
use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, trim_string,
};
use async_trait::async_trait;
use regex::Regex;

pub struct FourGTV;

#[async_trait]
impl Service for FourGTV {
    fn name(&self) -> String {
        "4GTV".to_string()
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
            "https://api2.4gtv.tv/Web/IsTaiwanArea",
            Some(four_gtv_headers()),
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

        let re = Regex::new(r#""Data":"([^"]+)"#).unwrap();

        let data = re.find(&html).unwrap().as_str();

        let data = trim_string(data, 8, 0);

        match data {
            "N" => UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            },
            "Y" => UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
            },
            _ => UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            },
        }
    }
}
