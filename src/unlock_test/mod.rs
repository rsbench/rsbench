pub mod headers;
mod unlock_script;
pub mod utils;

use crate::config::UnlockRegion;
use crate::unlock_test::unlock_script::{
    afr_services, all_services, asia_services, cn_services, euro_services, global_services,
    hk_services, jp_services, mo_services, tw_services, uk_services, us_services,
};
use crate::utils::{clear_last_line, set_colour, set_default_colour};
use async_trait::async_trait;
use futures::executor::block_on;
use std::fmt::{Display, Formatter};
use termcolor::Color;
use tokio::sync::mpsc;

#[derive(Debug)]
#[allow(dead_code)]
/// 表示解锁测试的结果
pub(crate) struct UnlockResult {
    /// 服务的名称
    pub service_name: String,
    /// 表示服务是否可用
    pub available: bool,
    /// 服务所在的区域，如果没有指定区域，则为 None
    pub region: Option<String>,
    /// 如果服务不可用，这里会包含错误信息，否则为 None
    pub error: Option<String>,
}

/// 定义一个异步 trait，用于表示一个服务
#[async_trait]
#[allow(dead_code)]
pub(crate) trait Service {
    /// 获取服务的名称
    fn name(&self) -> String;

    /// 异步检查服务是否解锁
    async fn check_unlock(&self) -> UnlockResult;
}

/// 异步函数，用于检查所有服务的解锁状态
pub async fn check_all(args: &crate::config::Config) {
    // 设置文本颜色为黄色
    set_colour(Color::Yellow);
    // 打印 UNLOCK 标题
    println!("UNLOCK:");
    // 打印表格标题
    println!("{:^5} {:^30} Error", "Y/N", "Service");
    // 设置文本颜色为默认颜色
    set_default_colour();

    // 获取需要测试的服务列表
    let services = get_test_service(args);
    // 获取服务列表的长度
    let services_count = services.len();

    // 创建一个 mpsc 通道，用于在异步任务之间传递消息
    let (tx, mut rx) = mpsc::channel(100);

    // 获取当前时间
    let time = tokio::time::Instant::now();

    // 遍历服务列表，为每个服务创建一个异步任务
    for service in services {
        // 克隆发送器，以便每个任务都有自己的发送器
        let tx = tx.clone();
        // 使用 tokio::spawn 创建一个异步任务，检查服务的解锁状态并发送结果
        tokio::spawn(async move {
            // 异步检查服务的解锁状态
            let result = service.check_unlock().await;
            // 将检查结果发送到通道中
            tx.send(result).await.unwrap();
        });
    }

    // 丢弃发送器，因为不再需要发送消息
    drop(tx);

    // 创建一个向量，用于存储接收到的结果
    let mut results = Vec::new();

    // 从通道中接收消息，直到通道关闭
    while let Some(result) = rx.recv().await {
        // 打印接收到的结果
        println!("{result}");
        // 将结果添加到结果向量中
        results.push(result);
    }

    // 计算测试所花费的时间
    let time = time.elapsed().as_secs_f64();

    // 打印分隔线
    for _ in 0..services_count {
        clear_last_line();
        // 等待 10 毫秒，以便打印效果更清晰
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    // 创建两个向量，分别用于存储已解锁和未解锁的服务
    let mut unlocked_services = Vec::new();
    let mut locked_services = Vec::new();

    // 遍历所有结果，将服务根据其解锁状态分类
    for result in &results {
        if result.available {
            unlocked_services.push(result);
        } else {
            locked_services.push(result);
        }
    }

    // 获取已解锁服务的数量
    let unlocked_services_count = unlocked_services.len();
    // 打印所有已解锁的服务
    for result in &unlocked_services {
        println!("{result}");
    }

    // 获取未解锁服务的数量
    let locked_services_count = locked_services.len();
    // 打印所有未解锁的服务
    for result in &locked_services {
        println!("{result}");
    }

    // 打印测试总结，包括测试的服务数量、花费的时间、已解锁和未解锁的服务数量
    println!(
        "Tested {services_count} projects took {time:.2} seconds, {unlocked_services_count} services unlocked, {locked_services_count} services locked.",
    );
}

impl Display for UnlockResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 设置文本颜色为默认颜色
        set_default_colour();

        // 根据服务是否可用，设置不同的文本颜色并格式化输出结果
        let result = if self.available {
            // 设置文本颜色为绿色
            set_colour(Color::Green);
            // 根据区域是否存在，格式化输出服务名称和区域
            match &self.region {
                // 如果区域不存在，直接输出服务名称
                None => {
                    write!(f, "[ Y ] {:^30}", self.service_name)
                }
                // 如果区域存在，输出服务名称和区域
                Some(region) => {
                    let service = format!("{} ({})", self.service_name, region);
                    write!(f, "[ Y ] {service:^30}")
                }
            }
        } else {
            // 设置文本颜色为红色
            set_colour(Color::Red);
            // 根据错误信息是否存在，格式化输出服务名称和错误信息
            match &self.error {
                // 如果错误信息不存在，直接输出服务名称
                None => {
                    write!(f, "[ N ] {:^30}", self.service_name)
                }
                // 如果错误信息存在，输出服务名称和错误信息
                Some(error) => {
                    // 尝试输出服务名称，如果失败则忽略
                    let _ = write!(f, "[ N ] {:^30}", self.service_name);
                    // 设置文本颜色为黄色
                    set_colour(Color::Yellow);
                    // 输出错误信息
                    write!(f, " {error}")
                }
            }
        };

        // 设置文本颜色为默认颜色
        set_default_colour();
        // 返回格式化结果
        result
    }
}

/// 获取需要测试的服务列表
pub fn get_test_service(args: &crate::config::Config) -> Vec<Box<dyn Service + Send + Sync>> {
    // 创建一个新的可变向量来存储服务
    let mut services = Vec::new();

    // 检查配置中是否指定了区域
    if args.region.is_none() {
        // 如果没有指定区域，则添加所有服务
        services.extend(all_services());
    } else {
        // 获取配置中的区域列表
        let regions = args.region.as_ref().unwrap();

        // 遍历区域列表
        for region in regions {
            // 根据区域添加相应的服务
            match region {
                UnlockRegion::HK => services.extend(hk_services()),
                UnlockRegion::MO => services.extend(mo_services()),
                UnlockRegion::TW => services.extend(tw_services()),
                UnlockRegion::JP => services.extend(jp_services()),
                UnlockRegion::CN => services.extend(cn_services()),
                UnlockRegion::Asia => services.extend(asia_services()),
                UnlockRegion::Euro => services.extend(euro_services()),
                UnlockRegion::Afr => services.extend(afr_services()),
                UnlockRegion::UK => services.extend(uk_services()),
                UnlockRegion::US => services.extend(us_services()),
                UnlockRegion::Global => services.extend(global_services()),
            }
        }
    }

    // 创建一个新的可变向量来存储去重后的服务
    let mut services_new = Vec::new();

    // 遍历服务列表，去除重复的服务
    for service in services {
        // 检查服务是否已经存在于新的服务列表中
        if services_new.contains(&service) {
            // 如果服务已经存在，则跳过
            continue;
        }
        // 将服务添加到新的服务列表中
        services_new.push(service);
    }

    // 返回去重后的服务列表
    services_new
}

/// 运行解锁测试的函数
pub fn run_unlock_test(args: &crate::config::Config) {
    // 使用 block_on 等待 check_all 函数执行完成
    block_on(check_all(args));
}
