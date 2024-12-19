// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::headers::bahamut_headers2;
use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, trim_string,
};
use async_trait::async_trait;
use regex::Regex;

pub struct BahamutAnime;

#[async_trait]
impl Service for BahamutAnime {
    fn name(&self) -> String {
        "Bahamut Anime".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client =
            match create_reqwest_client(self.name(), Some(super::utils::UA_BROWSER), true, None)
                .await
            {
                Ok(client) => client,
                Err(unlock_result) => return unlock_result,
            };

        let result_device_id = match get_url(
            self.name(),
            &client,
            "https://ani.gamer.com.tw/ajax/getdeviceid.php",
            None,
        )
        .await
        {
            Ok(result_device_id) => result_device_id,
            Err(unlock_result) => return unlock_result,
        };

        let html = match parse_response_to_html(self.name(), result_device_id).await {
            Ok(html) => html,
            Err(unlock_result) => return unlock_result,
        };

        let device_id = trim_string(&html, 13, 2);

        match get_url(
            self.name(),
            &client,
            &format!(
                "https://ani.gamer.com.tw/ajax/token.php?adID=89422&sn=37783&device={}",
                device_id
            ),
            None,
        )
        .await
        {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        let result2 = match get_url(
            self.name(),
            &client,
            "https://ani.gamer.com.tw/",
            Some(bahamut_headers2()),
        )
        .await
        {
            Ok(result2) => result2,
            Err(unlock_result) => return unlock_result,
        };

        let html2 = match parse_response_to_html(self.name(), result2).await {
            Ok(html2) => html2,
            Err(unlock_result) => return unlock_result,
        };

        let re = Regex::new(r#"data-geo="([A-Za-z]{2})""#).unwrap();

        let mut region = String::new();

        for line in re.find_iter(&html2) {
            region = trim_string(&line.as_str(), 10, 1)
                .to_string()
                .to_uppercase();
        }

        UnlockResult {
            service_name: self.name(),
            available: true,
            region: Some(region),
            error: None,
        }
    }
}
