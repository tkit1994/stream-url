use std::error::Error;

pub mod bilibili;

pub trait GetUrl {
    fn get_stream_url(&self) -> Result<Vec<String>, Box<dyn Error>>;
}
