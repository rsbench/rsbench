use crate::unlock_test::UnlockResult;
use reqwest::header::HeaderMap;
use reqwest::Client;
use std::time::Duration;

pub fn trim_string(s: &str, leading: usize, trailing: usize) -> &str {
    let start = leading;
    let end = s.len().saturating_sub(trailing);
    &s[start..end]
}

pub async fn create_reqwest_client(
    service_name: String,
    ua: Option<&str>,
    enable_cookies: bool,
    timeout: Option<u64>,
) -> Result<Client, UnlockResult> {
    let mut builder = Client::builder();
    if ua.is_some() {
        builder = builder.user_agent(ua.unwrap());
    }
    if enable_cookies {
        builder = builder.cookie_store(true);
    }
    if timeout.is_some() {
        builder = builder.timeout(Duration::from_secs(timeout.unwrap()));
    } else {
        builder = builder.timeout(Duration::from_secs(15));
    }

    match builder.build() {
        Ok(client) => Ok(client),
        Err(_) => Err(UnlockResult {
            service_name,
            available: false,
            region: None,
            error: Some("Can not initialize client".to_string()),
        }),
    }
}

pub const UA_BROWSER: &str = r#"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"#;

// Android
pub const UA_BROWSER2: &str = r#"Mozilla/5.0 (Linux; Android 10; Pixel 4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Mobile Safari/537.36"#;

pub async fn parse_response_to_html(
    service_name: String,
    result: reqwest::Response,
) -> Result<String, UnlockResult> {
    match result.text().await {
        Ok(html) => Ok(html),
        Err(_) => Err(UnlockResult {
            service_name,
            available: false,
            region: None,
            error: Some("Can not parse HTML".to_string()),
        }),
    }
}

pub async fn get_url(
    service_name: String,
    client: &Client,
    url: &str,
    header_map: Option<HeaderMap>,
) -> Result<reqwest::Response, UnlockResult> {
    let mut request = client.get(url);
    if header_map.is_some() {
        request = request.headers(header_map.unwrap());
    }

    let result = request.send().await;

    match result {
        Ok(result) => Ok(result),
        Err(_) => Err(UnlockResult {
            service_name,
            available: false,
            region: None,
            error: Some("Not available / Network connection error".to_string()),
        }),
    }
}
