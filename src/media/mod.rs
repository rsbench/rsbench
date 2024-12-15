mod hbomax;
mod netflix;
mod youtube_cdn;

use async_trait::async_trait;
use futures::{executor::block_on, future::join_all};

#[derive(Debug)]
#[allow(dead_code)]
struct UnlockResult {
    pub service_name: String,
    pub available: bool,
    pub region: Option<String>,
    pub error: Option<String>,
}

#[async_trait]
trait MediaService {
    fn name(&self) -> &'static str;

    async fn check_unlock(&self) -> UnlockResult;
}

pub async fn check_all() {
    let services: Vec<Box<dyn MediaService + Send + Sync>> = vec![
        Box::new(netflix::Netflix),
        Box::new(hbomax::HboMax),
        Box::new(youtube_cdn::YoutubeCDN),
    ];
    let futures = services.iter().map(|service| service.check_unlock());
    let results = join_all(futures).await;
    println!("{:?}", results);
}

pub fn run_media() {
    block_on(check_all());
}
