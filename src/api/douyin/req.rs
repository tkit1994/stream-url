use log::debug;
use reqwest::header::{HeaderMap, ACCEPT, REFERER, USER_AGENT};

use crate::api::GetUrl;

pub struct StreamRoom {
    pub url: String,
}

impl StreamRoom {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}

impl GetUrl for StreamRoom {
    fn get_stream_url(&self) -> anyhow::Result<Vec<String>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
                .parse()
                .unwrap(),
        );
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; WOW64; rv:51.0) Gecko/20100101 Firefox/51.0"
                .parse()
                .unwrap(),
        );
        headers.insert(REFERER, self.url.parse().unwrap());
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .unwrap();
        // send request twice to get cookies
        let _resp = client.get(&self.url).send()?;
        let resp = client.get(&self.url).send()?.text()?;
        debug!("{}", resp);
        let re = fancy_regex::Regex::new(
            r#"<script id="RENDER_DATA" type="application/json">(?P<content>.+?)</script>"#,
        )?;
        let cap = re.captures(&resp)?.unwrap();
        let content = cap.name("content").unwrap().as_str();
        let content = urlencoding::decode(content)?.to_string();
        debug!("{}", content);
        let re = fancy_regex::Regex::new(r#""FULL_HD1":"(?P<url>https?://.+?\.(flv|m3u8))""#)?;
        let cap = re.captures(content.as_str())?.unwrap();
        let url = cap.name("url").unwrap().as_str();
        let result = vec![url.to_string()];
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    // use env_logger::Env;

    use super::*;
    #[test]
    fn test_douyin() {
        // env_logger::Builder::from_env(Env::default().default_filter_or("DEBUG")).init();
        // let url = "https://live.douyin.com/868472687748";
        let url = "https://live.douyin.com/545551821556";
        let room = StreamRoom::new(url);
        let surl = room.get_stream_url().expect("");
        for i in surl {
            println!("{i}");
        }
    }
}
