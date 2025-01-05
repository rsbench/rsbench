use reqwest::Client;
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::time::Duration;

pub async fn create_reqwest_client(
    ua: Option<&str>,
    ipv6: bool, // FORCE
) -> Result<Client, ()> {
    let mut builder = Client::builder();

    builder = builder.timeout(Duration::from_secs(10));

    if ua.is_some() {
        builder = builder.user_agent(ua.unwrap());
    }

    if ipv6 {
        builder = builder.local_address(Some(IpAddr::V6(Ipv6Addr::from_str("::").unwrap())));
    } else {
        builder = builder.local_address(Some(IpAddr::V4(Ipv4Addr::from_str("0.0.0.0").unwrap())));
    }

    // 构建客户端
    match builder.build() {
        // 如果客户端构建成功，则返回 Ok(Client)
        Ok(client) => Ok(client),
        // 如果客户端构建失败，则返回 Err(UnlockResult)
        Err(_) => Err(()),
    }
}

pub fn json_value_to_string(json: &Value, key: &str) -> Option<String> {
    match json.get(key) {
        None => None,
        Some(value) => Some(value.as_str()?.to_string()),
    }
}
