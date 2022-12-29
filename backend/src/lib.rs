use anyhow::Context;
use async_trait::async_trait;

#[async_trait]
pub trait GetUrls {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>>;
}

#[async_trait]
pub trait GetUrl {
    async fn get_url(&self) -> anyhow::Result<String>;
}

#[async_trait]
impl<T: GetUrls + Sync> GetUrl for T {
    async fn get_url(&self) -> anyhow::Result<String> {
        let res = self
            .get_urls()
            .await?
            .get(0)
            .cloned()
            .context("No urls found");
        res
    }
}
pub mod bilibili;

pub mod douyu;
pub mod huya;
