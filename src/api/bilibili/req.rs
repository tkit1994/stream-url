
use anyhow::Result;
use crate::api::GetUrl;

use super::model::{PlayUrlResp, QNData};
use log::error;
pub struct StreamRoom {
    pub cid: i32,
    api_url: String,
    pub qn_data: QNData,
}

impl StreamRoom {
    pub fn new(cid: i32, qn_data: QNData) -> Self {
        Self {
            cid,
            api_url: "https://api.live.bilibili.com/room/v1/Room/playUrl".to_string(),
            qn_data,
        }
    }
}

impl GetUrl for StreamRoom {
    fn get_stream_url(&self) -> Result<Vec<String>> {
        let client = reqwest::blocking::Client::builder().build()?;

        let resp = client
            .get(&self.api_url)
            .query(&[
                ("cid", self.cid.to_string().as_str()),
                ("qn", (self.qn_data as i32).to_string().as_str()),
                ("plantform", "web"),
            ])
            .send()?
            .json::<PlayUrlResp>()?;
        if resp.code != 0 {
            error!("code: {}, message: {}", resp.code, resp.message);
        }
        let mut result = Vec::new();
        for i in resp.data.expect("No data").durl {
            result.push(i.url);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bilibili() {
        let room = StreamRoom::new(6750632, QNData::QSource);
        let url = room.get_stream_url().expect("Failed to get stream url");
        println!("{:?}", url);
    }
}
