use lazy_static::lazy_static;
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

// #[macro_export]
// macro_rules! global_println {
//     () => (global_print!("\n"));
//     ($fmt:expr) => (global_print!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (global_print!(concat!($fmt, "\n"), $($arg)*));
// }

#[macro_export]
macro_rules! global_println {
    ($($arg:tt)*) => {{
        let mut global_string = GLOBAL_STRING.lock().unwrap();
        writeln!(global_string, $($arg)*).expect("Failed to write to global string");
    }}
}
