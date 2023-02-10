use anyhow::{bail, Context};
use async_trait::async_trait;
use regex::Regex;

use crate::GetUrls;

pub struct StreamRoom {
    room_id: String,
    client: reqwest::Client,
}

impl StreamRoom {
    pub fn new(room_id: &str, client: reqwest::Client) -> Self {
        Self {
            room_id: room_id.to_owned(),
            client,
        }
    }
}

#[async_trait]
impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        let room_url = format!("https://www.huya.com/{}", self.room_id);
        let res = self.client.get(room_url).send().await?.text().await?;
        let re = Regex::new(r#""isOn":(?P<is_on>(true|false))"#)?;
        let is_on = re.captures(&res).context("isOn not found")?;
        match &is_on["is_on"] {
            "true" => {}
            "false" => bail!("Streaming has not started"),
            _ => bail!("Unknown error about is on"),
        }
        let re = Regex::new(r#"gameStreamInfoList":(?P<gameStreamInfoList>\[.*?}\])"#)?;
        let cap = re
            .captures(&res)
            .context("no captures for gameStreamInfoList")?;
        let info_list = &cap["gameStreamInfoList"];
        // println!("{}", info_list);
        let info_list = serde_json::from_str::<serde_json::Value>(info_list)?;
        let urls = info_list
            .as_array()
            .context("no urls")?
            .iter()
            .map(|f| {
                let s_flv_url = f["sFlvUrl"].as_str().unwrap().replace("http", "https");
                let s_stream_name = f["sStreamName"].as_str().unwrap();
                let s_flv_suffix = f["sFlvUrlSuffix"].as_str().unwrap();
                let s_flv_anti_code = f["sFlvAntiCode"].as_str().unwrap();
                format!("{s_flv_url}/{s_stream_name}.{s_flv_suffix}?{s_flv_anti_code}")
            })
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
        let room_id = "lck";
        let client = reqwest::Client::builder().build()?;
        let s = StreamRoom::new(room_id, client);
        let u = s.get_url().await?;
        println!("{u}");
        Ok(())
    }
}
