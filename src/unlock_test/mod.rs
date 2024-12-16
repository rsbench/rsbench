mod bahamut;
mod google_play_store;
mod hbomax;
mod iqiyi_oversea;
mod netflix;
mod steam;
mod utils;
mod youtube_cdn;
mod youtube_premium;

use async_trait::async_trait;
use futures::{executor::block_on, future::join_all};
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
    fn name(&self) -> &'static str;

    async fn check_unlock(&self) -> UnlockResult;
}

pub async fn check_all() {
    let mut log = paris::Logger::new();
    log.loading("Checking media services...");
    let services: Vec<Box<dyn Service + Send + Sync>> = vec![
        Box::new(netflix::Netflix),
//        Box::new(hbomax::HboMax),
        Box::new(youtube_cdn::YoutubeCDN),
        Box::new(youtube_premium::YoutubePremium),
        Box::new(google_play_store::GooglePlayStore),
        Box::new(iqiyi_oversea::IqiyiOversea),
        Box::new(steam::Steam),
        Box::new(bahamut::BahamutAnime),
    ];
    let futures = services.iter().map(|service| service.check_unlock());
    let results = join_all(futures).await;
    log.done();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for result in results {
        if result.available {
            if result.region.is_some() {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .unwrap();
                println!(
                    "{}",
                    format!("{}({})", result.service_name, result.region.unwrap())
                );
            } else {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                    .unwrap();
                println!("{}", format!("{}", result.service_name));
            }
        } else {
            if result.region.is_some() {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                if result.error.is_some() {
                    println!(
                        "{}",
                        format!(
                            "{}({}): {}",
                            result.service_name,
                            result.region.unwrap(),
                            result.error.unwrap()
                        )
                    );
                } else {
                    println!(
                        "{}",
                        format!("{}({})", result.service_name, result.region.unwrap())
                    );
                }
            } else {
                stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                if result.error.is_some() {
                    println!(
                        "{}",
                        format!("{}: {}", result.service_name, result.error.unwrap())
                    );
                } else {
                    println!("{}", format!("{}", result.service_name));
                }
            }
        }
    }
}

pub fn run_media() {
    block_on(check_all());
}
