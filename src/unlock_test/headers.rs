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

pub fn lemino_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "accept",
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert("origin", "https://lemino.docomo.ne.jp".parse().unwrap());
    headers.insert("referer", "https://lemino.docomo.ne.jp/".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        r#""Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24""#
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers.insert(
        "x-service-token",
        "f365771afd91452fa279863f240c233d".parse().unwrap(),
    );
    headers.insert(
        "x-trace-id",
        "556db33f-d739-4a82-84df-dd509a8aa179".parse().unwrap(),
    );
    headers
}

pub fn animefesta_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Origin", "https://animefesta.iowl.jp".parse().unwrap());
    headers.insert("Priority", "u=1, i".parse().unwrap());
    headers.insert("Referer", "https://animefesta.iowl.jp/".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "same-site".parse().unwrap());
    headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    headers
}

pub fn mora_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("host", "mora.jp".parse().unwrap());
    headers
}

pub fn dazn_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers
}

pub fn hulu_jp_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "*/*;q=0.8".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
    headers.insert("Accept-Encoding", "none".parse().unwrap());
    headers.insert("Sec-GPC", "1".parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    headers.insert("Sec-Fetch-Dest", "document".parse().unwrap());
    headers.insert("Sec-Fetch-Mode", "navigate".parse().unwrap());
    headers.insert("Sec-Fetch-Site", "none".parse().unwrap());
    headers.insert("Sec-Fetch-User", "?1".parse().unwrap());
    headers.insert("Priority", "u=1".parse().unwrap());
    headers
}

pub fn nowe_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers
}

pub fn viutv_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers
}

pub fn unext_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers
}

pub fn four_gtv_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("origin", "https://www.4gtv.tv".parse().unwrap());
    headers.insert("referer", "https://www.4gtv.tv/".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        r#""Google Chrome";v="125", "Chromium";v="125", "Not.A/Brand";v="24""#
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers
}

pub fn showmax_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("host", "www.showmax.com".parse().unwrap());
    headers.insert("connection", "keep-alive".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Chromium\";v=\"124\", \"Microsoft Edge\";v=\"124\", \"Not-A.Brand\";v=\"99\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
    headers
}
