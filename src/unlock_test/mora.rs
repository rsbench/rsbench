// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::headers::mora_headers;
use crate::unlock_test::utils::{create_reqwest_client, get_url};
use async_trait::async_trait;

pub struct Mora;

#[async_trait]
impl Service for Mora {
    fn name(&self) -> String {
        "Mora".to_string()
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
            "https://mora.jp/buy?__requestToken=1713764407153&returnUrl=https%3A%2F%2Fmora.jp%2Fpackage%2F43000087%2FTFDS01006B00Z%2F%3Ffmid%3DTOPRNKS%26trackMaterialNo%3D31168909&fromMoraUx=false&deleteMaterial=",
            Some(mora_headers()),
            None,
        )
            .await
        {
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
        } else if result.status().as_u16() == 403 {
            UnlockResult {
                service_name: self.name(),
                available: false,
                region: None,
                error: None,
            }
        } else if result.status().as_u16() == 500 {
            UnlockResult {
                service_name: self.name(),
                available: false,
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
