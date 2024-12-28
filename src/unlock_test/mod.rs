mod animefesta;
mod bahamut;
mod bbc_iplayer;
mod bilibili;
mod dazn;
mod four_gtv;
mod google_play_store;
mod hami_video;
mod hbomax;
mod headers;
mod hulu_jp;
mod iqiyi_oversea;
mod kancolle;
mod lemino;
mod mora;
mod mytv_super;
mod netflix;
mod nowe;
mod princess_connect_redive_japan;
mod sling_tv;
mod steam;
mod unext;
mod utils;
mod viutv;
mod youtube_cdn;
mod youtube_premium;

use crate::utils::{clear_last_line, set_colour, set_default_colour};
use async_trait::async_trait;
use futures::executor::block_on;
use std::fmt::{Display, Formatter};
use termcolor::Color;
use tokio::sync::mpsc;

#[derive(Debug)]
#[allow(dead_code)]
struct UnlockResult {
    pub service_name: String,
    pub available: bool,
    pub region: Option<String>,
    pub error: Option<String>,
}

#[async_trait]
#[allow(dead_code)]
trait Service {
    fn name(&self) -> String;

    async fn check_unlock(&self) -> UnlockResult;
}

pub async fn check_all() {
    //    let mut log = paris::Logger::new();
    //    log.loading("Checking media services...");

    set_colour(Color::Yellow);
    println!("UNLOCK:");
    println!("{:^5} {:^30} {}", "Y/N", "Service", "Error");
    set_default_colour();

    let services: Vec<Box<dyn Service + Send + Sync>> = vec![
        Box::new(netflix::Netflix),
        Box::new(hbomax::HboMax),
        Box::new(youtube_cdn::YoutubeCDN),
        Box::new(youtube_premium::YoutubePremium),
        Box::new(google_play_store::GooglePlayStore),
        Box::new(iqiyi_oversea::IqiyiOversea),
        Box::new(steam::Steam),
        Box::new(bahamut::BahamutAnime),
        Box::new(bilibili::BilibiliChinaMainland),
        Box::new(bilibili::BilibiliChinaTWOnly),
        Box::new(bilibili::BilibiliChinaHKMOTW),
        Box::new(princess_connect_redive_japan::PrincessConnectReDiveJapan),
        Box::new(kancolle::Kancolle),
        Box::new(lemino::Lemino),
        Box::new(animefesta::AnimeFesta),
        Box::new(mora::Mora),
        Box::new(bbc_iplayer::BBCIPlayer),
        Box::new(dazn::Dazn),
        Box::new(hulu_jp::HuluJP),
        Box::new(mytv_super::MyTVSuper),
        Box::new(nowe::NowE),
        Box::new(viutv::ViuTV),
        Box::new(unext::UNext),
        Box::new(four_gtv::FourGTV),
        Box::new(sling_tv::SlingTV),
        Box::new(hami_video::HamiVideo),
    ];

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

pub fn run_unlock_test() {
    block_on(check_all());
}
