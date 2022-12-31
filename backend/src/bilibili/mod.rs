use anyhow::Context;
use async_trait::async_trait;
use serde_json::json;

use crate::GetUrls;

pub struct StreamRoom {
    room_id: u64,
    client: reqwest::Client,
    api_url: String,
    qn_data: u64,
}

impl StreamRoom {
    pub fn new(room_id: u64, client: reqwest::Client, qn_data: u64) -> Self {
        let api_url = "https://api.live.bilibili.com/room/v1/Room/playUrl".to_owned();
        Self {
            room_id,
            client,
            api_url,
            qn_data,
        }
    }
}

#[async_trait]
impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        let query = json!({
            "cid": self.room_id,
            "qn": self.qn_data,
            "plantform": "web"
        });
        let res = self
            .client
            .get(&self.api_url)
            .query(&query)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let urls = res["data"]["durl"]
            .as_array()
            .context("No durl")?
            .iter()
            .flat_map(|f| f["url"].as_str().map(|f| f.to_owned()))
            .collect::<Vec<_>>();
        Ok(urls)
    }
}

#[cfg(test)]
mod tests {
    use crate::GetUrl;

    use super::*;
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let client = reqwest::Client::builder().build()?;
        // let room_id = 21743919;
        let room_id = 1;
        let s = StreamRoom::new(room_id, client, 10000);
        let url = s.get_url().await?;
        println!("{}", url);
        Ok(())
    }
}
