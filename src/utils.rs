use crate::unlock_test::Service;
use rand::Rng;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// 设置终端输出的文本颜色为随机颜色
pub fn set_random_colour() {
    let no_color = std::env::var("RSBENCH_NO_COLOR").unwrap_or("0".to_string());
    if no_color == "1" {
        return;
    }

    // 定义一个内部函数 random_colour，用于生成随机颜色
    fn random_colour() -> Color {
        // 创建一个颜色数组，包含多种颜色
        let colours = [
            Color::White,
            Color::Green,
            Color::Red,
            Color::Black,
            Color::Blue,
            Color::Cyan,
            Color::Magenta,
            Color::Yellow,
        ];

        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();
        // 生成一个随机索引，范围在颜色数组的长度内
        let random_index = rng.gen_range(0..colours.len());
        // 返回随机索引对应的颜色
        colours[random_index]
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
pub fn _clear_last_line() {
    // 打印 ANSI 转义序列，将光标向上移动一行
    print!("\x1b[A");
    // 打印 ANSI 转义序列，清除当前行的内容
    print!("\x1b[2K");
    // 打印回车符，将光标移动到行首
    print!("\r");
    // 刷新标准输出，确保所有字符都被打印出来
    std::io::stdout().flush().unwrap();
}

pub fn clear_screen() {
    print!("\x1b[2J");
    std::io::stdout().flush().unwrap();
}

// 为 Box<dyn Service + Send + Sync> 实现 PartialEq trait，用于比较两个动态盒子是否相等
impl PartialEq for Box<dyn Service + Send + Sync> {
    // 定义 eq 方法，比较两个动态盒子的名称是否相等
    fn eq(&self, other: &Self) -> bool {
        // 调用 name 方法获取服务名称，并比较是否相等
        self.name() == other.name()
    }
}
