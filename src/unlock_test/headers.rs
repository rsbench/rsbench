// 该文件用于储存各个平台的 Headers 信息
//
// 请勿在检测文件内直接写一长串 Headers
// 而是转为在本文件定义函数并返回 Headers
//
// 函数名 请以 服务检测的文件名 为前缀

use reqwest::header;
use reqwest::header::HeaderMap;

pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
    headers.insert("cache-control", "max-age=0".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("priority", "u=0, i".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".parse().unwrap());
    headers
}

pub fn bahamut_headers2() -> HeaderMap {
    let mut headers = HeaderMap::new();
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

pub fn google_play_store_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "en-US;q=0.9".parse().unwrap());
    headers.insert("priority", "u=0, i".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\", \"Google Chrome\";v=\"131\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".parse().unwrap());
    headers
}

pub fn netflix_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert("host", "www.netflix.com".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        r#""Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24""#
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers
}

pub fn youtube_premium_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert(header::COOKIE, "YSC=FSCWhKo2Zgw; VISITOR_PRIVACY_METADATA=CgJERRIEEgAgYQ%3D%3D; PREF=f7=4000; __Secure-YEC=CgtRWTBGTFExeV9Iayjele2yBjIKCgJERRIEEgAgYQ%3D%3D; SOCS=CAISOAgDEitib3FfaWRlbnRpdHlmcm9udGVuZHVpc2VydmVyXzIwMjQwNTI2LjAxX3AwGgV6aC1DTiACGgYIgMnpsgY; VISITOR_INFO1_LIVE=Di84mAIbgKY; __Secure-BUCKET=CGQ".parse().unwrap());
    headers
}
