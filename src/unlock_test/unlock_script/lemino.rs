// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::headers::lemino_headers;
use crate::unlock_test::utils::{create_reqwest_client, get_url, UA_BROWSER};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;
use reqwest::Body;

pub struct Lemino;

#[async_trait]
impl Service for Lemino {
    fn name(&self) -> String {
        "Lemino".to_string()
    }

    async fn check_unlock(&self) -> UnlockResult {
        let client = match create_reqwest_client(self.name(), Some(UA_BROWSER), true, None).await {
            Ok(client) => client,
            Err(unlock_result) => return unlock_result,
        };
        let result = match get_url(self.name(), &client, "https://if.lemino.docomo.ne.jp/v1/user/delivery/watch/ready", Some(lemino_headers()), Some(Body::from("{\"inflow_flows\":[null,\"crid://plala.iptvf.jp/group/b100ce3\"],\"play_type\":1,\"key_download_only\":null,\"quality\":null,\"groupcast\":null,\"avail_status\":\"1\",\"terminal_type\":3,\"test_account\":0,\"content_list\":[{\"kind\":\"main\",\"service_id\":null,\"cid\":\"00lm78dz30\",\"lid\":\"a0lsa6kum1\",\"crid\":\"crid://plala.iptvf.jp/vod/0000000000_00lm78dymn\",\"preview\":0,\"trailer\":0,\"auto_play\":0,\"stop_position\":0}]}"))).await {
            Ok(result) => result,
            Err(unlock_result) => return unlock_result,
        };

        if result.status().as_u16() == 200 {
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
                error: None,
            }
        }
    }
}
