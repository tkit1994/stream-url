use anyhow::Context;

use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

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

impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        let url = format!("https://live.douyin.com/{}", self.room_id);
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, HeaderValue::from_str("__ac_nonce=0;")?);
        let res = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(
            r#"<script id="RENDER_DATA" type="application/json">(?P<content>.+?)</script>"#,
        )?;
        let cap = re.captures(res.as_str()).context("no caps")?;
        let content = &cap["content"];
        let content = urlencoding::decode(content)?.to_string();
        let re = Regex::new(r#""FULL_HD1":"(?P<url>https?://.+?\.(flv|m3u8).*?)""#)?;
        let cap = re.captures(content.as_str()).context("no caps")?;
        let url = cap["url"].to_string();
        let res = vec![url];
        Ok(res)
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
        // let room_id = 669559769472;
        let room_id = 816599815264;
        let s = StreamRoom::new(room_id, client);
        let url = s.get_url().await?;
        println!("{url}");
        Ok(())
    }
}
