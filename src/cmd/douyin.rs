
use clap::{ArgGroup, Args};
use stream_url::api::{self};

use super::get_url;
#[derive(Debug, Args)]
#[clap(group(
            ArgGroup::new("id")
                .required(true)
        ))]
pub struct DouyinArgs {
    #[clap(short, long, group = "id")]
    url: Option<String>,
    // #[clap(short, long, group = "id")]
    // room_id: Option<i64>,
    #[clap(short, long)]
    all: bool,
}

pub fn execute(args: DouyinArgs) -> anyhow::Result<()> {
    if let Some(url) = args.url {
        let room = api::douyin::StreamRoom::new(&url);
        get_url(room, args.all)?
    }
    Ok(())
}
