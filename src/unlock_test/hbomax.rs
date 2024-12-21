// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh
// 该脚本稍微有点小问题，不论什么区域检测都会显示 US 区域解锁，尚不清楚原因
// Disabled

use super::{Service, UnlockResult};
use crate::unlock_test::utils::{create_reqwest_client, get_url, parse_response_to_html};
use async_trait::async_trait;
use regex::Regex;

pub struct HboMax;

#[async_trait]
impl Service for HboMax {
    fn name(&self) -> String {
        "HBO MAX".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client =
            match create_reqwest_client(self.name(), Some(super::utils::UA_BROWSER), false, None)
                .await
            {
                Ok(client) => client,
                Err(unlock_result) => return unlock_result,
            };

        let result = match get_url(self.name(), &client, "https://www.max.com/", None, None).await {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        if result.status().as_u16() != 200 {
            return UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some("Not available".to_string()),
            };
        }

        let html = match parse_response_to_html(self.name(), result).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        let mut country_list = Vec::new();
        let re = Regex::new(r#""url":"/[a-z]{2}/[a-z]{2}""#).unwrap(); // "url":"/tr/en"

        for line in re.find_iter(&html) {
            country_list
                .push(crate::unlock_test::utils::trim_string(line.as_str(), 8, 4).to_uppercase());
        }

        let mut unique_vec: Vec<_> = country_list.iter().collect();
        unique_vec.sort();
        unique_vec.dedup();

        let country_list = unique_vec;

        let re = Regex::new(r#""countryCode":"[a-z]{2}""#).unwrap();

        let region_country_code = match re.find(&html) {
            None => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
            Some(country) => country.as_str(),
        };

        let region =
            crate::unlock_test::utils::trim_string(region_country_code, 15, 1).to_uppercase();

        if country_list.contains(&&region) {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: Some(region),
                error: None,
            }
        } else {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: Some(region),
                error: Some(std::string::String::from(
                    "Not available / Network connection error",
                )),
            }
        }
    }
}
