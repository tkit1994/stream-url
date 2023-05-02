use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use async_trait::async_trait;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;

use crate::GetUrls;

pub struct StreamRoom {
    room_id: u64,
    client: reqwest::Client,
    cdns: Vec<String>,
}

impl StreamRoom {
    pub fn new(room_id: u64, client: reqwest::Client) -> Self {
        let cdns = vec![
            "hw-tct.douyucdn.cn".to_owned(),
            "hdltc1.douyucdn.cn".to_owned(),
            "hdltctwk.douyucdn2.cn".to_owned(),
        ];
        Self {
            room_id,
            client,
            cdns,
        }
    }
}

#[async_trait]
impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let auth = md5::compute(format!("{}{}", self.room_id, ts));
        let auth = format! {"{auth:x}"};

        let body = json!({
            "rid": self.room_id,
            "did": "10000000000000000000000000001501"
        });
        let mut headers = HeaderMap::new();
        headers.insert("rid", HeaderValue::from(self.room_id));
        headers.insert("time", HeaderValue::from(ts));
        headers.insert("auth", HeaderValue::from_str(auth.as_str()).unwrap());
        let url = format!(
            "https://playweb.douyucdn.cn/lapi/live/hlsH5Preview/{}",
            self.room_id
        );
        let res = self
            .client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let urls = res["data"]["rtmp_live"].as_str().context(res.to_string())?;
        let re = Regex::new(r"(?P<key>\d{1,8}[0-9a-zA-Z]+)_?\d{0,4}(.m3u8|/playlist)")?;
        let cap = re.captures(urls).context("no caps for key")?;
        let key = &cap["key"];
        let res = self
            .cdns
            .iter()
            .map(|cdn| format!("https://{cdn}/live/{key}.m3u8?uuid="))
            .collect::<Vec<_>>();
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
        let room_id = 12313;
        let s = StreamRoom::new(room_id, client);
        let url = s.get_url().await?;
        println!("{url}");
        Ok(())
    }
}
