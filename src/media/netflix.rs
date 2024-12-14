use super::{MediaService, UnlockResult};
use async_trait::async_trait;

pub struct Netflix;

#[async_trait]
impl MediaService for Netflix {
    async fn check_unlock(&self) -> UnlockResult {
        UnlockResult {
            service_name: "Netflix".to_string(),
            available: true,
            region: Some("US".to_string()),
            error: None,
        }
    }
}
