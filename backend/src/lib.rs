#![allow(async_fn_in_trait)]
use anyhow::{bail, Context};

pub trait GetUrls {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>>;
}

pub trait GetUrl {
    async fn get_url(&self) -> anyhow::Result<String>;
}

impl<T: GetUrls + Sync> GetUrl for T {
    async fn get_url(&self) -> anyhow::Result<String> {
        let res = self
            .get_urls()
            .await?
            .first()
            .cloned()
            .context("No urls found");
        res
    }
}
mod bilibili;

mod douyin;
mod douyu;
mod huya;
mod twitch;

pub enum StreamRoom {
    Douyu(douyu::StreamRoom),
    Huya(huya::StreamRoom),
    Bilibili(bilibili::StreamRoom),
    Douyin(douyin::StreamRoom),
    Twitch(twitch::StreamRoom),
    Unknownplatform,
}

impl StreamRoom {
    pub fn new(platform: &str, room_id: &str, client: reqwest::Client) -> StreamRoom {
        match platform.to_lowercase().as_str() {
            "douyu" => {
                let room_id = room_id.parse().unwrap();
                let room = douyu::StreamRoom::new(room_id, client);
                StreamRoom::Douyu(room)
            }
            "huya" => {
                let room = huya::StreamRoom::new(room_id, client);
                StreamRoom::Huya(room)
            }
            "bilibili" => {
                let room_id = room_id.parse().unwrap();
                let room = bilibili::StreamRoom::new(room_id, client, 10000);
                StreamRoom::Bilibili(room)
            }
            "douyin" => {
                let room_id = room_id.parse().unwrap();
                let room = douyin::StreamRoom::new(room_id, client);
                StreamRoom::Douyin(room)
            }
            "twitch" => {
                let room = twitch::StreamRoom::new(room_id, client);
                StreamRoom::Twitch(room)
            }
            _ => StreamRoom::Unknownplatform,
        }
    }
}
impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        match self {
            StreamRoom::Douyu(room) => room.get_urls().await,
            StreamRoom::Huya(room) => room.get_urls().await,
            StreamRoom::Bilibili(room) => room.get_urls().await,
            StreamRoom::Douyin(room) => room.get_urls().await,
            StreamRoom::Twitch(room) => room.get_urls().await,
            _ => bail!("Unknownplatform"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let client = reqwest::Client::builder().build()?;
        let room_id = "6556593";
        let room = StreamRoom::new("douyu", room_id, client);
        let url = room.get_url().await?;
        println!("{url}");
        Ok(())
    }
}
