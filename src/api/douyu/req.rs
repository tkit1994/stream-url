use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use log::{debug};

use crate::api::GetUrl;

pub struct StreamRoom {
    pub roomd_id: i32,
    api_url: String,
    did: String,
    ts: String,
    cdns: Vec<String>,
}

impl StreamRoom {
    pub fn new(roomd_id: i32) -> Self {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).expect("");
        let cdns = vec![
            "dyscdnali1.douyucdn.cn".to_string(),
            "dyscdnali3.douyucdn.cn".to_string(),
            "hls3-akm.douyucdn.cn".to_string(),
            "hlsa-akm.douyucdn.cn".to_string(),
            "hls1a-akm.douyucdn.cn".to_string(),
        ];
        Self {
            roomd_id,
            api_url: "https://m.douyu.com/api/room/ratestream".to_string(),
            did: "10000000000000000000000000001501".to_string(),
            ts: ts.as_secs().to_string(),
            cdns,
        }
    }
    pub fn get_js(&self) -> Result<String, Box<dyn Error>> {
        let client = reqwest::blocking::Client::builder().build()?;

        let req = client
            .get(format!("{}/{}", "https://m.douyu.com", self.roomd_id))
            .send()?
            .text()?;
        debug!("{}", req);
        let re = fancy_regex::Regex::new(
            r#"(?P<ub98484234>function ub98484234.*+)\s(?P<vc4db0801>var.*)"#,
        )
        .expect("");

        let cap = re.captures(&req)?.unwrap();
        let ub98484234 = cap.name("ub98484234").unwrap().as_str();

        let vc4db0801 = cap.name("vc4db0801").unwrap().as_str();
        let re = fancy_regex::Regex::new(r#"eval.*;}"#).expect("");
        let ub98484234 = re.replace(ub98484234, "strc;}").to_string();
        let mut js =
            js_sandbox::Script::from_string(&format!("{} {}", ub98484234, vc4db0801)).expect("");
        let func_sign: String = js.call("ub98484234", &"")?;
        let re = fancy_regex::Regex::new(r#"v=(?P<v>\d+)"#)?;

        let cap = re.captures(&func_sign)?.unwrap();
        let v = cap.name("v").unwrap().as_str();
        let rb = format!("{}{}{}{}", self.roomd_id, self.did, self.ts, v);
        let rb = md5::compute(rb.as_bytes());
        let rb = format! {"{:x}", rb};
        debug!("{}", rb);

        let re = fancy_regex::Regex::new(r#"return rt;}\);?"#)?;
        let func_sign = re.replace(&func_sign, "return rt;}").to_string();
        let re = fancy_regex::Regex::new(r#"\(function \("#)?;
        let func_sign = re.replace(&func_sign, r#"function sign("#).to_string();
        let re = fancy_regex::Regex::new(r#"CryptoJS.MD5\(cb\).toString\(\)"#)?;
        let func_sign = re.replace(&func_sign, format!("\"{}\"", rb)).to_string();
        let re = fancy_regex::Regex::new(r#"function sign\(xx0,xx1,xx2\){"#)?;
        let func_sign = re
            .replace(
                &func_sign,
                r#"function sign(arg){var xx0=arg[0];var xx1=arg[1];var xx2=arg[2];"#,
            )
            .to_string();
        debug!("{}", func_sign);
        let mut func_sign = js_sandbox::Script::from_string(&func_sign)?;
        let resp: String =
            func_sign.call("sign", &[&self.roomd_id.to_string(), &self.did, &self.ts])?;
        debug!("{}", resp);
        Ok(resp)
    }
}

impl GetUrl for StreamRoom {
    fn get_stream_url(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder().build()?;
        let params = self.get_js()?;
        let params = format!("{}&ver=219032101&rid={}&rate=-1", params, self.roomd_id);
        let params = serde_urlencoded::from_str::<Vec<(String, String)>>(&params)?;
        debug!("{:?}", params);
        let resp = client.post(&self.api_url).query(&params).send()?.text()?;
        debug!("{resp}");
        let re =
            fancy_regex::Regex::new(r#"(?P<key>\d{1,8}[0-9a-zA-Z]+)_?\d{0,4}(.m3u8|/playlist)"#)?;
        let cap = re.captures(&resp)?.unwrap();
        let key = cap.name("key").unwrap().as_str();
        let mut result = Vec::new();
        for cdn in &self.cdns {
            result.push(format!("http://{cdn}/live/{key}.flv?uuid="));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_get_js() {
        let room = StreamRoom::new(6872425);
        let js = room.get_js().expect("");
        println!("{}", js);
    }
    #[test]
    fn test_get_url() {
        let room = StreamRoom::new(6012419);
        for i in room.get_stream_url().expect("") {
            println!("{i}");
        }
    }
}
