use std::error::Error;

pub mod bilibili;
pub mod huya;
pub trait GetUrl {
    fn get_stream_url(&self) -> Result<Vec<String>, Box<dyn Error>>;
}
