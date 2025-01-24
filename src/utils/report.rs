use lazy_static::lazy_static;
use paris::error;
use regex::Regex;
use reqwest::Client;
use std::sync::Mutex;
use std::time::Duration;

pub async fn get_usage_count() -> Result<(u64, u64), String> {
    let client = Client::new();
    let text = match client.get("https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Frsbench%2Frsbench&count_bg=%23000000&title_bg=%23FF0000&icon=rust.svg&icon_color=%2300FF5D&title=Call+times&edge_flat=false")
        .timeout(Duration::from_secs(2)).send().await {
        Ok(res) => {
            match res.text().await {
                Ok(text) => text,
                Err(_) => {
                    return Err("Can not parse response".to_string())
                }
            }
        }
        Err(_) => {
            return Err("Can not parse response".to_string())
        }
    };

    let re = Regex::new(r"\d+\s/\s\d+").unwrap();
    let line = if let Some(text) = re.find(&text) {
        text.as_str()
    } else {
        return Err("Can not parse response".to_string());
    };

    let vec = line.split('/').collect::<Vec<&str>>();

    Ok((
        vec[0].trim().parse::<u64>().unwrap(),
        vec[1].trim().parse::<u64>().unwrap(),
    ))
}

lazy_static! {
    pub static ref GLOBAL_STRING: Mutex<String> = Mutex::new(String::new());
}

#[macro_export]
macro_rules! global_print {
    ($($arg:tt)*) => {{
        let mut global_string = GLOBAL_STRING.lock().unwrap();
        write!(global_string, $($arg)*).expect("Failed to write to global string");
    }}
}

#[macro_export]
macro_rules! global_println {
    ($($arg:tt)*) => {{
        let mut global_string = GLOBAL_STRING.lock().unwrap();
        writeln!(global_string, $($arg)*).expect("Failed to write to global string");
    }}
}

#[allow(clippy::await_holding_lock)]
pub async fn post_to_pastebin() -> Result<String, String> {
    // https://pastebin.highp.ing
    let url = if let Some(url) = option_env!("PASTEBIN_URL") {
        url
    } else {
        error!("Compiling without specifying `PASTEBIN_URL` will now skip Pastebin uploads");
        return Err(
            "Compiling without specifying `PASTEBIN_URL` will now skip Pastebin uploads"
                .to_string(),
        );
    };

    // If you see this password, please do not share it with others. (๑•̀ㅂ•́)و✧
    let secret = if let Some(secret) = option_env!("PASTEBIN_SECRET") {
        secret
    } else {
        error!("Compiling without specifying `PASTEBIN_SECRET` will now skip Pastebin uploads");
        return Err(
            "Compiling without specifying `PASTEBIN_SECRET` will now skip Pastebin uploads"
                .to_string(),
        );
    };

    let client = Client::new();
    let resp = client
        .post(format!("{}/upload", url))
        .header("Authorization", secret)
        .body(GLOBAL_STRING.lock().unwrap().clone())
        .send()
        .await;
    let text = if let Ok(res) = resp {
        if !res.status().is_success() {
            return Err("You have no permission to upload".to_string());
        }
        match res.text().await {
            Ok(text) => text,
            Err(_) => return Err("Can not parse response".to_string()),
        }
    } else {
        return Err("Can not parse response".to_string());
    };

    let id = text.trim().parse::<u64>().unwrap();
    Ok(format!("{url}/{id}"))
}
