use crate::api::{huya::model::HuyaResp, GetUrl};
use anyhow::Result;
use log::debug;
use reqwest::header::{HeaderMap, ACCEPT, REFERER, USER_AGENT};

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
    fn get_stream_url(&self) -> Result<Vec<String>> {
        let client = reqwest::blocking::Client::builder().build().unwrap();
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

        let resp = client.get(&self.url).headers(headers).send()?.text()?;
        let resp = html_escape::decode_html_entities(&resp).to_string();
        let re = fancy_regex::Regex::new(r"(?ms)hyPlayerConfig = (?P<cfg>{.*?});")?;
        let cap = re.captures(&resp)?.unwrap();
        let hy_player_config = cap.name("cfg").unwrap().as_str();
        debug!("{hy_player_config}");
        let re = fancy_regex::Regex::new(r#""stream": "(?P<stream>.*?)""#)?;
        let cap = re.captures(hy_player_config)?.unwrap();
        let stream = cap.name("stream").unwrap().as_str();
        let stream = base64::decode(stream).unwrap();
        let stream = serde_json::from_slice::<HuyaResp>(&stream)?;
        debug!("{stream:?}");
        let mut result = Vec::new();
        for data in stream.data {
            debug!("{:?}", data.game_stream_info_list);
            for info in data.game_stream_info_list {
                let url = format!(
                    "{}/{}.flv{}",
                    info.s_flv_url,
                    info.s_stream_name,
                    html_escape::decode_html_entities(&info.s_flv_anti_code)
                );
                result.push(url);
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_huya() {
        let url = "https://www.huya.com/lck";
        let room = StreamRoom::new(url);
        let surl = room.get_stream_url().expect("");
        for i in surl {
            println!("{i}");
        }
    }
}
