use log::error;

use crate::api::{self, GetUrl};

pub enum Plantform {
    Bilibili,
    Huya,
    Douyu,
}

pub fn get_stream_room(plantform: Plantform, roomid: i64) -> impl GetUrl {
    match plantform {
        Plantform::Bilibili => {
            api::bilibili::req::StreamRoom::new(roomid, api::bilibili::model::QNData::QSource)
        }
        // Plantform::Huya => {},
        // Plantform::Douyu => {},
        _ => {
            error!("Unknow plantform");
            todo!()
        }
    }
}
