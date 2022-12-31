use anyhow::Context;
use async_trait::async_trait;
use regex::Regex;
use reqwest::header::{HeaderValue, USER_AGENT};

use crate::GetUrls;

pub struct StreamRoom {
    room_id: u64,
    client: reqwest::Client,
}

impl StreamRoom {
    pub fn new(room_id: u64, client: reqwest::Client) -> Self {
        Self { room_id, client }
    }
}

#[async_trait]
impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        let room_url = format!("https://m.huya.com/{}", self.room_id);
        let res = self.client.get(room_url)
		.header(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Linux; Android 5.0; SM-G900P Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.100 Mobile Safari/537.36").unwrap())
		.send().await?.text().await?;
        let re = Regex::new(r#"liveLineUrl":[\s\S]*?"(?P<url>.+?)""#)?;
        let cap = re.captures(&res).context("no captures for liveLineUrl")?;
        let url = &cap["url"];
        let url = base64::decode(url)?;
        let url = String::from_utf8(url)?;
        let url = format!("https:{}", url);
        let url = url
            .replace("m3u8", "flv")
            .replace("hls", "flv")
            .replace("tars_mobile", "huya_live");
        let urls = vec![url];
        Ok(urls)
    }
}

#[cfg(test)]
mod tests {
    use crate::GetUrl;

    use super::*;
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let room_id = 213517;
        let client = reqwest::Client::builder().build()?;
        let s = StreamRoom::new(room_id, client);
        let u = s.get_url().await?;
        println!("{}", u);
        Ok(())
    }
}
