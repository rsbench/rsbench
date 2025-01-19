use crate::unlock_test::Service;
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};
use rand::Rng;
use regex::Regex;
use reqwest::Client;
use std::io::stdout;
use std::time::Duration;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// 设置终端输出的文本颜色为随机颜色
pub fn set_random_colour() {
    let no_color = std::env::var("RSBENCH_NO_COLOR").unwrap_or("0".to_string());
    if no_color == "1" {
        return;
    }

    // 定义一个内部函数 random_colour，用于生成随机颜色
    fn random_colour() -> Color {
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();
        Color::Rgb(
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        )
    }

    // 获取标准输出的句柄，并指定颜色选择为 Always
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    // 设置标准输出的颜色为随机生成的颜色
    stdout
        // 创建一个新的颜色配置，设置前景色为随机颜色
        .set_color(ColorSpec::new().set_fg(Some(random_colour())))
        // 如果设置颜色失败，抛出异常
        .unwrap();
}

/// 设置终端输出的文本颜色为默认颜色（白色）
pub fn set_default_colour() {
    let no_color = std::env::var("RSBENCH_NO_COLOR").unwrap_or("0".to_string());
    if no_color == "1" {
        return;
    }
    // 获取标准输出的句柄，并指定颜色选择为 Always
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    // 设置标准输出的颜色为默认颜色（白色）
    stdout
        // 创建一个新的颜色配置，设置前景色为白色
        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        // 如果设置颜色失败，抛出异常
        .unwrap();
}

/// 设置终端输出的文本颜色
pub fn set_colour(color: Color) {
    let no_color = std::env::var("RSBENCH_NO_COLOR").unwrap_or("0".to_string());
    if no_color == "1" {
        return;
    }
    // 获取标准输出的句柄，并指定颜色选择为 Always
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    // 设置标准输出的颜色为指定的颜色
    stdout
        // 创建一个新的颜色配置，设置前景色为指定的颜色
        .set_color(ColorSpec::new().set_fg(Some(color)))
        // 如果设置颜色失败，抛出异常
        .unwrap();
}

/// 清除终端中的最后一行文本
pub fn clear_last_line() {
    execute!(
        stdout(),
        cursor::MoveUp(1),
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine)
    )
    .unwrap();
}

pub fn clear_screen() {
    execute!(
        stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}

// 为 Box<dyn Service + Send + Sync> 实现 PartialEq trait，用于比较两个动态盒子是否相等
impl PartialEq for Box<dyn Service + Send + Sync> {
    // 定义 eq 方法，比较两个动态盒子的名称是否相等
    fn eq(&self, other: &Self) -> bool {
        // 调用 name 方法获取服务名称，并比较是否相等
        self.name() == other.name()
    }
}

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
