// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use async_trait::async_trait;
use reqwest::Client;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Linux; Android 10; Pixel 4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Mobile Safari/537.36"#;
pub struct PrincessConnectReDiveJapan;

#[async_trait]
impl Service for PrincessConnectReDiveJapan {
    fn name(&self) -> String {
        "Princess Connect Re:Dive Japan".to_string()
    }
    async fn check_unlock(&self) -> UnlockResult {
        let client = match Client::builder().user_agent(UA_BROWSER).build() {
            Ok(client) => client,
            Err(_) => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not initialize client")),
                };
            }
        };
        let result = match client
            .get("https://api-priconne-redive.cygames.jp/")
            .send()
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Not available / Network connection error")),
                }
            }
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
