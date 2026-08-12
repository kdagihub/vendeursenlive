use async_trait::async_trait;

use crate::{application::errors::ApplicationError, domain::value_objects::TikTokLiveUrl};

#[async_trait]
pub trait TikTokLiveLinkResolver: Send + Sync {
    async fn resolve(&self, shared_url: &str) -> Result<TikTokLiveUrl, ApplicationError>;
}
