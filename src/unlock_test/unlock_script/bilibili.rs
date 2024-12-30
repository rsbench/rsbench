// https://github.com/lmc999/RegionRestrictionCheck/blob/main/check.sh

use crate::unlock_test::utils::{
    create_reqwest_client, get_url, parse_response_to_html, trim_string, UA_BROWSER,
};
use crate::unlock_test::{Service, UnlockResult};
use async_trait::async_trait;
use regex::Regex;
use std::cmp::Ordering;

const MAINLAND_URL: &str = r"https://api.bilibili.com/pgc/player/web/playurl?avid=82846771&qn=0&type=&otype=json&ep_id=307247&fourk=1&fnver=0&fnval=16&session=2964df126ad2f9d834dd4fda26fe1061&module=bangumi";
const TW_URL: &str = r"https://api.bilibili.com/pgc/player/web/playurl?avid=50762638&cid=100279344&qn=0&type=&otype=json&ep_id=268176&fourk=1&fnver=0&fnval=16&session=2964df126ad2f9d834dd4fda26fe1061&module=bangumi";
const HKMOTW_URL: &str = r"https://api.bilibili.com/pgc/player/web/playurl?avid=18281381&cid=29892777&qn=0&type=&otype=json&ep_id=183799&fourk=1&fnver=0&fnval=16&session=2964df126ad2f9d834dd4fda26fe1061&module=bangumi";

async fn get_bilibili_url(name: String, url: String) -> UnlockResult {
    let client = match create_reqwest_client(name.clone(), Some(UA_BROWSER), false, None).await {
        Ok(client) => client,
        Err(unlock_result) => return unlock_result,
    };

    let result = match get_url(name.clone(), &client, &url, None, None).await {
        Ok(result) => result,
        Err(unlock_result) => return unlock_result,
    };

    let html = match parse_response_to_html(name.clone(), result).await {
        Ok(html) => html,
        Err(unlock_result) => return unlock_result,
    };

    let re = Regex::new(r#""code":-?\d+"#).unwrap();

    let line = match re.find(&html) {
        None => {
            return UnlockResult {
                service_name: name,
                available: false,
                region: None,
                error: Some(String::from("Can not get response status code")),
            }
        }
        Some(line) => line.as_str(),
    };

    let code = trim_string(line, 7, 0).to_string().parse::<i32>().unwrap();

    match code.cmp(&0) {
        Ordering::Less => UnlockResult {
            service_name: name,
            available: false,
            region: None,
            error: Some(String::from("Not available")),
        },
        Ordering::Equal => UnlockResult {
            service_name: name,
            available: true,
            region: None,
            error: None,
        },
        Ordering::Greater => UnlockResult {
            service_name: name,
            available: false,
            region: None,
            error: None,
        },
    }
}

pub struct BilibiliChinaMainland;
pub struct BilibiliChinaTWOnly;
pub struct BilibiliChinaHKMOTW;

#[async_trait]
impl Service for BilibiliChinaMainland {
    fn name(&self) -> String {
        String::from("Bilibili China Mainland")
    }

    async fn check_unlock(&self) -> UnlockResult {
        get_bilibili_url(self.name(), MAINLAND_URL.to_string()).await
    }
}

#[async_trait]
impl Service for BilibiliChinaTWOnly {
    fn name(&self) -> String {
        String::from("Bilibili China TW Only")
    }

    async fn check_unlock(&self) -> UnlockResult {
        get_bilibili_url(self.name(), TW_URL.to_string()).await
    }
}

#[async_trait]
impl Service for BilibiliChinaHKMOTW {
    fn name(&self) -> String {
        String::from("Bilibili China HK/MO/TW")
    }

    async fn check_unlock(&self) -> UnlockResult {
        get_bilibili_url(self.name(), HKMOTW_URL.to_string()).await
    }
}
