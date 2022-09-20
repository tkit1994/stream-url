use anyhow::Result;
pub mod bilibili;
pub mod douyin;
pub mod douyu;
pub mod huya;
pub trait GetUrl {
    fn get_stream_url(&self) -> Result<Vec<String>>;
}
