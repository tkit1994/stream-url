use anyhow::Result;
use clap::{ArgGroup, Args};
use stream_url::api::bilibili::{self, model::QNData};

use super::get_url;
#[derive(Debug, Args)]
#[clap(group(
            ArgGroup::new("id")
                .required(true)
        ))]
pub struct BilibiliArgs {
    // #[clap(short, long, group = "id")]
    // url: Option<String>,
    #[clap(short, long, group = "id")]
    room_id: Option<i64>,
    #[clap(short, long, arg_enum, default_value_t=QNData::QSource)]
    qn_data: QNData,
    #[clap(short, long)]
    all: bool,
}
pub fn execute(args: BilibiliArgs) -> Result<()> {
    let qn_data = args.qn_data;
    if let Some(room_id) = args.room_id {
        let room = bilibili::StreamRoom::new(room_id, qn_data);
        get_url(room, args.all)?;
    }
    Ok(())
}
