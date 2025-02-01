use rand::Rng;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn set_random_colour() {
    let no_color = std::env::var("RSBENCH_NO_COLOR").unwrap_or("0".to_string());
    if no_color == "1" {
        return;
    }

    // 定义一个内部函数 random_colour，用于生成随机颜色
    fn random_colour() -> Color {
        // 创建一个随机数生成器
        let mut rng = rand::rng();
        Color::Rgb(
            rng.random_range(0..255),
            rng.random_range(0..255),
            rng.random_range(0..255),
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
