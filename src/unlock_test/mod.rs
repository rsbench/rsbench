mod animefesta;
mod bahamut;
mod bilibili;
mod google_play_store;
mod hbomax;
mod headers;
mod iqiyi_oversea;
mod kancolle;
mod lemino;
mod mora;
mod netflix;
mod princess_connect_redive_japan;
mod steam;
mod utils;
mod youtube_cdn;
mod youtube_premium;

use async_trait::async_trait;
use futures::executor::block_on;
use std::fmt::{Display, Formatter};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
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

    let services: Vec<Box<dyn Service + Send + Sync>> = vec![
        Box::new(netflix::Netflix),
        //Box::new(hbomax::HboMax),
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
    ];

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

    while let Some(result) = rx.recv().await {
        println!("{}", result);
    }

    println!("Time taken: {:?}", time.elapsed());
}

impl Display for UnlockResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 创建一个标准输出流，并设置颜色选择为 Always
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        // 设置前景色为白色
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .unwrap();

        // 根据 available 字段的值来决定输出内容
        let result = if self.available {
            // 设置前景色为绿色
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            // 根据 region 字段是否存在来决定输出内容
            match &self.region {
                None => {
                    // 输出服务名称
                    write!(f, "{}", self.service_name)
                }
                Some(region) => {
                    // 输出服务名称和地区
                    write!(f, "{} ({})", self.service_name, region)
                }
            }
        } else {
            // 设置前景色为红色
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            // 根据 error 字段是否存在来决定输出内容
            match &self.error {
                None => {
                    // 输出服务名称
                    write!(f, "{}", self.service_name)
                }
                Some(error) => {
                    // 输出服务名称和错误信息
                    write!(f, "{}: {}", self.service_name, error)
                }
            }
        };

        // 恢复前景色为白色
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .unwrap();
        // 返回格式化结果
        result
    }
}

pub fn run_unlock_test() {
    block_on(check_all());
}
