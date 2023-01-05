use anyhow::Context;
use async_trait::async_trait;
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
        let query = r#"
        query PlaybackAccessToken_Template(
          $login: String!,
          $isLive: Boolean!,
          $vodID: ID!,
          $isVod: Boolean!,
          $playerType: String!
        ) {
          streamPlaybackAccessToken(
            channelName: $login,
            params: {
              platform: "web",
              playerBackend: "mediaplayer",
              playerType: $playerType
            }
          ) @include(if: $isLive) {
            value
            signature
            __typename
          }
          videoPlaybackAccessToken(
            id: $vodID,
            params: {
              platform: "web",
              playerBackend: "mediaplayer",
              playerType: $playerType
            }
          ) @include(if: $isVod) {
            value
            signature
            __typename
          }
        }"#;

        let variables = json!({
            "isLive": true,
            "login": self.room_id.as_str(),
            "isVod": false,
            "vodID": "",
            "playerType": "site",
        });

        let request_body = json!({
            "query": query,
            "variables": variables,
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

#[async_trait]
impl GetUrls for StreamRoom {
    async fn get_urls(&self) -> anyhow::Result<Vec<String>> {
        let (token, signature) = self.sign_token().await?;
        let params = json!({
                    "allow_source": "true",
                    "dt": 2,
                    "fast_bread": "true",
                    "player_backend": "mediaplayer",
                    "playlist_include_framerate": "true",
                    "reassignments_supported": "true",
                    "sig": signature,
                    "supported_codecs": "vp09,avc1",
                    "token": token,
                    "cdm": "wv",
                    "player_version": "1.4.0",
        });
        let params = serde_urlencoded::to_string(params)?;
        let url = format!(
            "https://usher.ttvnw.net/api/channel/hls/{}.m3u8?{}",
            self.room_id, params
        );
        let m3u8 = self.client.get(url).send().await?.text().await?;
        // println!("{}", m3u8);
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
        let room_id = "velvet_7";
        let client = reqwest::Client::builder().build()?;
        let r = StreamRoom::new(room_id, client);
        let url = r.get_url().await?;
        println!("{}", url);
        Ok(())
    }
}
