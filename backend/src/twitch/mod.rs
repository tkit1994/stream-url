use anyhow::Context;

use regex::Regex;
use serde_json::json;

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

    async fn sign_token(&self) -> anyhow::Result<(String, String)> {
        let request_body = json!({
            "operationName": "PlaybackAccessToken",
            "extensions": {
                "persistedQuery": {
                    "version": 1,
                    "sha256Hash": "0828119ded1c13477966434e15800ff57ddacf13ba1911c129dc2200705b0712"
                }
            },
            "variables": {
                "isLive": true,
                "login": self.room_id,
                "isVod": false,
                "vodID": "",
                "playerType": "embed"
            }
        });
        let res = self
            .client
            .post("https://gql.twitch.tv/gql")
            .header("Client-ID", "kimne78kx3ncx6brgo4mv6wki5h1ko")
            .json(&request_body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let token = res["data"]["streamPlaybackAccessToken"]["value"]
            .as_str()
            .context(res.to_string())?;
        let signature = res["data"]["streamPlaybackAccessToken"]["signature"]
            .as_str()
            .context(res.to_string())?;
        Ok((token.to_owned(), signature.to_owned()))
    }
}

impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        let (token, signature) = self.sign_token().await?;
        let params = json!({
            "sig": signature,
            "token": token,
            "player": "twitchweb",
            "p": 123456,
            "type": "any",
            "allow_source": "true",
            "allow_audio_only": "true",
            "allow_spectre": "false",
        });
        let params = serde_urlencoded::to_string(params)?;
        let url = format!(
            "https://usher.ttvnw.net/api/channel/hls/{}.m3u8?{}",
            self.room_id, params
        );
        let m3u8 = self.client.get(url).send().await?.text().await?;
        let re = Regex::new(r"(?P<url>https?://.*)")?;
        let urls = re
            .captures_iter(m3u8.as_str())
            .map(|f| f["url"].to_owned())
            .collect::<Vec<_>>();
        if urls.is_empty() {
            anyhow::bail!(m3u8);
        }
        Ok(urls)
    }
}
#[cfg(test)]
mod tests {
    use crate::GetUrl;

    use super::*;
    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let room_id = "weiwei610";
        let client = reqwest::Client::builder().build()?;
        let r = StreamRoom::new(room_id, client);
        let url = r.get_url().await?;
        println!("{url}");
        Ok(())
    }
}
