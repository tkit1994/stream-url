use clap::ArgEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayUrlResp {
    pub code: i64,
    pub message: String,
    pub ttl: i64,
    pub data: Option<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub current_quality: i64,
    pub accept_quality: Vec<String>,
    pub current_qn: i64,
    pub quality_description: Vec<QualityDescription>,
    pub durl: Vec<Durl>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Durl {
    pub url: String,
    pub length: i64,
    pub order: i64,
    pub stream_type: i64,
    #[serde(rename = "p2p_type")]
    pub p2_p_type: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityDescription {
    pub qn: i64,
    pub desc: String,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum QNData {
    QSource = 10000,
    QBlue = 400,
    QSuper = 250,
    QHigh = 150,
}
