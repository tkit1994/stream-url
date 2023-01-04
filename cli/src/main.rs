use backend::GetUrl;
use clap::Parser;
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short)]
    platform: String,
    #[arg(long, short)]
    room_id: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::builder().build()?;
    let room = backend::StreamRoom::new(args.platform.as_str(), args.room_id.as_str(), client);
    let url = room.get_url().await?;
    println!("{}", url);
    Ok(())
}
