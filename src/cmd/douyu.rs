use clap::{ArgGroup, Args};
use stream_url::api::{self};

use super::get_url;

#[derive(Debug, Args)]
#[clap(group(
            ArgGroup::new("id")
                .required(true)
        ))]
pub struct DouyuArgs {
    // #[clap(short, long, group = "id")]
    // url: Option<String>,
    #[clap(short, long, group = "id")]
    room_id: Option<i64>,
    #[clap(short, long)]
    all: bool,
}
pub fn execute(args: DouyuArgs) -> anyhow::Result<()> {
    if let Some(room_id) = args.room_id {
        let room = api::douyu::StreamRoom::new(room_id);
        get_url(room, args.all)?
    }
    Ok(())
}
