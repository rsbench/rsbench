mod bahamut;
mod bilibili;
mod google_play_store;
mod hbomax;
mod headers;
mod iqiyi_oversea;
mod netflix;
mod princess_connect_redive_japan;
mod steam;
mod utils;
mod youtube_cdn;
mod youtube_premium;

use async_trait::async_trait;
use futures::{executor::block_on, future::join_all};
use std::fmt::{Display, Formatter};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
    let mut log = paris::Logger::new();
    log.loading("Checking media services...");

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
    ];

    let futures = services.iter().map(|service| service.check_unlock());
    let results = join_all(futures).await;
    log.done();

    for result in results {
        println!("{}", result);
    }
}

impl Display for UnlockResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .unwrap();
        let result = if self.available {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
            match &self.region {
                None => {
                    write!(f, "{}", self.service_name)
                }
                Some(region) => {
                    write!(f, "{} ({})", self.service_name, region)
                }
            }
        } else {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            match &self.error {
                None => {
                    write!(f, "{}", self.service_name)
                }
                Some(error) => {
                    write!(f, "{}: {}", self.service_name, error)
                }
            }
        };

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .unwrap();
        result
    }
}

pub fn run_unlock_test() {
    block_on(check_all());
}
