// 该文件用于储存各个平台的 Headers 信息
//
// 请勿在检测文件内直接写一长串 Headers
// 而是转为在本文件定义函数并返回 Headers
//
// 函数名 请以 服务检测的文件名 为前缀

use reqwest::header;
use reqwest::header::HeaderMap;

pub fn bahamut_headers2() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "accept",
        "*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
            .parse()
            .unwrap(),
    );
    headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        r#""Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24""#
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-model", "\"\"".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-ch-ua-platform-version", "\"15.0.0\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers
}
