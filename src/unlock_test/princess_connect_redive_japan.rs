// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::utils::{create_reqwest_client, get_url};
use async_trait::async_trait;

pub struct PrincessConnectReDiveJapan;

#[async_trait]
impl Service for PrincessConnectReDiveJapan {
    fn name(&self) -> String {
        "Princess Connect Re:Dive Japan".to_string()
    }
    async fn check_unlock(&self) -> UnlockResult {
        let client =
            match create_reqwest_client(self.name(), Some(super::utils::UA_BROWSER2), false, None)
                .await
            {
                Ok(client) => client,
                Err(unlock_result) => return unlock_result,
            };
        let result = match get_url(
            self.name(),
            &client,
            "https://api-priconne-redive.cygames.jp/",
            None,
        )
        .await
        {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };
        if result.status().as_u16() == 404 {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
            }
        } else if result.status().as_u16() == 403 {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Not available")),
            }
        } else {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: Some(String::from("Network connection error")),
            }
        }
    }
}
