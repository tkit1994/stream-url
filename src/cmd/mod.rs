use clap::{Parser, Subcommand};
use stream_url::api::{GetUrl};

pub mod bilibili;
pub mod douyu;
pub mod huya;
pub mod douyin;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = "A tool to get stream urls")]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Bilibili
    Bilibili(bilibili::BilibiliArgs),
    /// Douyu
    Douyu(douyu::DouyuArgs),
    /// Huya
    Huya(huya::HuyaArgs),
    /// Douyin
    Douyin(douyin::DouyinArgs)
}

pub fn get_url(room: impl GetUrl, all: bool) -> anyhow::Result<()> {
    let urls = room.get_stream_url()?;
    if all {
        for i in urls {
            println!("{}", i);
        }
    } else {
        println!("{}", urls[0]);
    }
    Ok(())
}
