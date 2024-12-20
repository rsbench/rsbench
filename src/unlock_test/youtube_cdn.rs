// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, UA_BROWSER,
};
use async_trait::async_trait;
use regex::Regex;

pub struct YoutubeCDN;

#[async_trait]
impl Service for YoutubeCDN {
    fn name(&self) -> String {
        "Youtube CDN".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), false, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };

        let result = match get_url(
            self.name(),
            &client,
            "https://redirector.googlevideo.com/report_mapping",
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

        let re = Regex::new(r#"=>"#).unwrap();

        let mut first_line = "";
        for line in html.lines() {
            if re.is_match(line) {
                first_line = line;
                break;
            }
        }

        let binding = first_line.split_whitespace().collect::<Vec<&str>>();
        let cdn_node = match binding.get(2) {
            None => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not get CDN node")),
                }
            }
            Some(cdn_node) => cdn_node,
        };

        let cdn_node = if cdn_node.contains("-") {
            let binding = cdn_node.split("-").collect::<Vec<&str>>();
            match binding.get(1) {
                None => {
                    return UnlockResult {
                        service_name: self.name(),
                        available: false,
                        region: None,
                        error: Some(String::from("Can not get CDN node")),
                    }
                }
                Some(cdn_node) => cdn_node.to_uppercase(),
            }
        } else {
            cdn_node.to_uppercase()
        };

        let mut count = 0;
        let mut cdn_region = String::new();
        for char in cdn_node.chars() {
            if count == 3 {
                break;
            }
            cdn_region.push(char);
            count += 1;
        }

        UnlockResult {
            service_name: self.name(),
            available: true,
            region: Some(cdn_region),
            error: None,
        }
    }
}
