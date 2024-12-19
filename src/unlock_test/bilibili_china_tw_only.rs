// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use super::{Service, UnlockResult};
use crate::unlock_test::utils::trim_string;
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;

const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;

pub struct BilibiliChinaTWOnly;

#[async_trait]
impl Service for BilibiliChinaTWOnly {
    fn name(&self) -> String {
        "Bilibili China TW Only".to_string()
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
        let result = match client.get("https://api.bilibili.com/pgc/player/web/playurl?avid=50762638&cid=100279344&qn=0&type=&otype=json&ep_id=268176&fourk=1&fnver=0&fnval=16&session=2964df126ad2f9d834dd4fda26fe1061&module=bangumi").send().await {
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

        let html = match result.text().await {
            Ok(html) => html,
            Err(_) => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not parse HTML")),
                }
            }
        };

        let re = Regex::new(r#""code":-?\d+"#).unwrap();

        let line = match re.find(&html) {
            None => {
                return UnlockResult {
                    service_name: self.name(),
                    available: false,
                    region: None,
                    error: Some(String::from("Can not get response status code")),
                }
            }
            Some(line) => line.as_str(),
        };

        let code = trim_string(line, 7, 0).to_string().parse::<i32>().unwrap();

        if code == 0 {
            UnlockResult {
                service_name: self.name(),
                available: true,
                region: None,
                error: None,
            }
        } else if code < 0 {
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
                error: None,
            }
        }
    }
}
