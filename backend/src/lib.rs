use anyhow::{bail, Context};
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
mod bilibili;

mod douyu;
mod huya;

pub enum StreamRoom {
    Douyu(douyu::StreamRoom),
    Huya(huya::StreamRoom),
    Bilibili(bilibili::StreamRoom),
    UnknownPlantform,
}

impl StreamRoom {
    pub fn new(plantform: &str, room_id: u64, client: reqwest::Client) -> StreamRoom {
        match plantform.to_lowercase().as_str() {
            "douyu" => {
                let room = douyu::StreamRoom::new(room_id, client);
                StreamRoom::Douyu(room)
            }
            "huya" => {
                let room = huya::StreamRoom::new(room_id, client);
                StreamRoom::Huya(room)
            }
            "bilibili" => {
                let room = bilibili::StreamRoom::new(room_id, client, 10000);
                StreamRoom::Bilibili(room)
            }
            _ => StreamRoom::UnknownPlantform,
        }
    }
}
#[async_trait]
impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        match self {
            StreamRoom::Douyu(room) => room.get_urls().await,
            StreamRoom::Huya(room) => room.get_urls().await,
            StreamRoom::Bilibili(room) => room.get_urls().await,
            StreamRoom::UnknownPlantform => bail!("UnknownPlantform"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let client = reqwest::Client::builder().build()?;
        let room_id = 6556593;
        let room = StreamRoom::new("douyu", room_id, client);
        let url = room.get_url().await?;
        println!("{}", url);
        Ok(())
    }
}
