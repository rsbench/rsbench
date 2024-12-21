use crate::unlock_test::UnlockResult;
use reqwest::header::HeaderMap;
use reqwest::{Body, Client};
use std::time::Duration;

/// 从字符串的开头和结尾修剪指定数量的字符
///
/// # 参数
/// * `s` - 要修剪的字符串
/// * `leading` - 要从字符串开头修剪的字符数
/// * `trailing` - 要从字符串结尾修剪的字符数
///
/// # 返回值
/// * 修剪后的字符串切片
pub fn trim_string(s: &str, leading: usize, trailing: usize) -> &str {
    // 计算修剪后的字符串的起始索引
    let start = leading;
    // 计算修剪后的字符串的结束索引，使用 saturating_sub 方法避免索引越界
    let end = s.len().saturating_sub(trailing);
    // 返回修剪后的字符串切片
    &s[start..end]
}

/// 创建一个 reqwest 客户端
///
/// # 参数
/// * `service_name` - 服务名称
/// * `ua` - 用户代理字符串，可选
/// * `enable_cookies` - 是否启用 cookie 存储，默认值为 false
/// * `timeout` - 请求超时时间，可选
///
/// # 返回值
/// * `Ok(Client)` - 如果客户端创建成功
/// * `Err(UnlockResult)` - 如果客户端创建失败
pub async fn create_reqwest_client(
    service_name: String,
    ua: Option<&str>,
    enable_cookies: bool,
    timeout: Option<u64>,
) -> Result<Client, UnlockResult> {
    // 创建一个 reqwest 客户端构建器
    let mut builder = Client::builder();
    // 如果用户代理字符串存在，则设置用户代理
    if ua.is_some() {
        builder = builder.user_agent(ua.unwrap());
    }
    // 如果启用 cookie 存储，则设置 cookie 存储
    if enable_cookies {
        builder = builder.cookie_store(true);
    }
    // 如果超时时间存在，则设置超时时间
    if timeout.is_some() {
        builder = builder.timeout(Duration::from_secs(timeout.unwrap()));
    } else {
        // 如果超时时间不存在，则设置默认超时时间为 15 秒
        builder = builder.timeout(Duration::from_secs(15));
    }

    // 构建客户端
    match builder.build() {
        // 如果客户端构建成功，则返回 Ok(Client)
        Ok(client) => Ok(client),
        // 如果客户端构建失败，则返回 Err(UnlockResult)
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

/// 将响应解析为 HTML 字符串
///
/// # 参数
/// * `service_name` - 服务名称
/// * `result` - 要解析的响应
///
/// # 返回值
/// * `Ok(String)` - 如果解析成功，返回 HTML 字符串
/// * `Err(UnlockResult)` - 如果解析失败，返回错误信息
pub async fn parse_response_to_html(
    service_name: String,
    result: reqwest::Response,
) -> Result<String, UnlockResult> {
    // 尝试将响应解析为文本
    match result.text().await {
        // 如果解析成功，返回 HTML 字符串
        Ok(html) => Ok(html),
        // 如果解析失败，返回错误信息
        Err(_) => Err(UnlockResult {
            service_name,
            available: false,
            region: None,
            error: Some("Can not parse HTML".to_string()),
        }),
    }
}

/// 发送 HTTP GET 请求到指定的 URL
///
/// # 参数
/// * `service_name` - 服务名称
/// * `client` - 用于发送请求的 reqwest 客户端
/// * `url` - 要请求的 URL
/// * `header_map` - 可选的请求头映射
/// * `body` - 可选的请求体
///
/// # 返回值
/// * `Ok(reqwest::Response)` - 如果请求成功，返回响应
/// * `Err(UnlockResult)` - 如果请求失败，返回错误信息
pub async fn get_url(
    service_name: String,
    client: &Client,
    url: &str,
    header_map: Option<HeaderMap>,
    body: Option<Body>,
) -> Result<reqwest::Response, UnlockResult> {
    // 创建一个 GET 请求
    let mut request = client.get(url);
    // 如果存在 header_map，则将其添加到请求中
    if header_map.is_some() {
        request = request.headers(header_map.unwrap());
    }
    if body.is_some() {
        request = request.body(body.unwrap());
    }

    // 发送请求并等待响应
    let result = request.send().await;

    // 匹配响应结果
    match result {
        // 如果响应成功，返回响应
        Ok(result) => Ok(result),
        // 如果响应失败，返回错误信息
        Err(_) => Err(UnlockResult {
            service_name,
            available: false,
            region: None,
            error: Some("Not available / Network connection error".to_string()),
        }),
    }
}
