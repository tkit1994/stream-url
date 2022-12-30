use backend::GetUrl;
use clap::Parser;
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short)]
    plantform: String,
    #[arg(long, short)]
    room_id: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::builder().build()?;
    let room = backend::StreamRoom::new(args.plantform.as_str(), args.room_id, client);
    let url = room.get_url().await?;
    println!("{}", url);
    Ok(())
}
