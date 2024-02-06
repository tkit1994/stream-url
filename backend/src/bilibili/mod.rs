use anyhow::Context;

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
        let api_url =
            "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo".to_owned();
        Self {
            room_id,
            client,
            api_url,
            qn_data,
        }
    }
}

impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        let query = json!({
            "room_id": self.room_id,
            "format": "0,1,2",
            "protocol": "0,1",
            "codec": "0,1",
            "qn": self.qn_data,
        });
        let res = self
            .client
            .get(&self.api_url)
            .query(&query)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let res = res["data"]["playurl_info"]["playurl"]["stream"]
            .as_array()
            .context("Streaming is not started")?
            .iter()
            .flat_map(|stream| {
                stream["format"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|&f| f["format_name"].as_str() != Some("flv"))
                    .flat_map(|f| {
                        f["codec"].as_array().unwrap().iter().flat_map(|codec| {
                            let base_url = codec["base_url"].as_str().unwrap();
                            let url_info = codec["url_info"].as_array().unwrap();
                            url_info.iter().map(move |u| {
                                let host = u["host"].as_str().unwrap();
                                let extra = u["extra"].as_str().unwrap();
                                format!("{host}{base_url}{extra}")
                            })
                        })
                    })
            })
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
        // let room_id = 21743919;
        let room_id = 21507954;
        let s = StreamRoom::new(room_id, client, 10000);
        let url = s.get_url().await?;
        println!("{url}");
        Ok(())
    }
}
