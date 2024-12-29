pub mod headers;
mod unlock_script;
mod utils;

use crate::config::UnlockRegion;
use crate::unlock_test::unlock_script::*;
use crate::utils::{clear_last_line, set_colour, set_default_colour};
use async_trait::async_trait;
use futures::executor::block_on;
use std::fmt::{Display, Formatter};
use termcolor::Color;
use tokio::sync::mpsc;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct UnlockResult {
    pub service_name: String,
    pub available: bool,
    pub region: Option<String>,
    pub error: Option<String>,
}

#[async_trait]
#[allow(dead_code)]
pub(crate) trait Service {
    fn name(&self) -> String;

    async fn check_unlock(&self) -> UnlockResult;
}

pub async fn check_all(args: &crate::config::Config) {
    set_colour(Color::Yellow);
    println!("UNLOCK:");
    println!("{:^5} {:^30} {}", "Y/N", "Service", "Error");
    set_default_colour();

    let services = get_test_service(&args);

    let services_count = services.len();

    let (tx, mut rx) = mpsc::channel(100);

    let time = tokio::time::Instant::now();

    for service in services {
        let tx = tx.clone();
        tokio::spawn(async move {
            let result = service.check_unlock().await;
            tx.send(result).await.unwrap();
        });
    }

    drop(tx);

    let mut results = Vec::new();
    while let Some(result) = rx.recv().await {
        println!("{}", result);
        results.push(result);
    }

    let time = time.elapsed().as_secs_f64();

    for _ in 0..services_count {
        clear_last_line();
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    let mut unlocked_services = Vec::new();
    let mut locked_services = Vec::new();
    for result in results {
        if result.available {
            unlocked_services.push(result);
        } else {
            locked_services.push(result);
        }
    }
    let unlocked_services_count = unlocked_services.len();
    let locked_services_count = locked_services.len();

    for result in unlocked_services {
        println!("{}", result);
    }
    for result in locked_services {
        println!("{}", result);
    }

    println!(
        "Tested {} projects took {:.2} seconds, {} services unlocked, {} services locked.",
        services_count, time, unlocked_services_count, locked_services_count,
    );
}
impl Display for UnlockResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        set_default_colour();

        // 根据 available 字段的值来决定输出内容
        let result = if self.available {
            set_colour(Color::Green);
            // 根据 region 字段是否存在来决定输出内容
            match &self.region {
                None => {
                    // 输出服务名称
                    write!(f, "[ Y ] {:^30}", self.service_name)
                }
                Some(region) => {
                    // 输出服务名称和地区
                    let service = format!("{} ({})", self.service_name, region);
                    write!(f, "[ Y ] {:^30}", service)
                }
            }
        } else {
            // 设置前景色为红色
            set_colour(Color::Red);
            // 根据 error 字段是否存在来决定输出内容
            match &self.error {
                None => {
                    // 输出服务名称
                    write!(f, "[ N ] {:^30}", self.service_name)
                }
                Some(error) => {
                    // 输出服务名称和错误信息
                    match write!(f, "[ N ] {:^30}", self.service_name) {
                        _ => {}
                    };
                    set_colour(Color::Yellow);
                    write!(f, " {}", error)
                }
            }
        };

        // 恢复前景色为白色
        set_default_colour();
        // 返回格式化结果
        result
    }
}

pub fn get_test_service(args: &crate::config::Config) -> Vec<Box<dyn Service + Send + Sync>> {
    let mut services = Vec::new();
    if !args.region.is_some() {
        services.extend(all_services());
    } else {
        let regions = args.region.as_ref().unwrap();
        for region in regions {
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

    let mut services_new = Vec::new();

    for service in services {
        if services_new.contains(&service) {
            continue;
        }
        services_new.push(service);
    }

    services_new
}

impl PartialEq for Box<dyn Service + Send + Sync> {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

pub fn run_unlock_test(args: &crate::config::Config) {
    block_on(check_all(args));
}
