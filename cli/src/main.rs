use backend::GetUrl;
use clap::{Parser, ValueEnum};
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short, value_enum)]
    plantform: Plantform,
    #[arg(long, short)]
    room_id: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Plantform {
    Huya,
    Bilibili,
    Douyu,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::builder().build()?;
    let room: Box<dyn GetUrl> = match args.plantform {
        Plantform::Huya => Box::new(backend::huya::StreamRoom::new(args.room_id, client)),
        Plantform::Bilibili => Box::new(backend::bilibili::StreamRoom::new(
            args.room_id,
            client,
            10000,
        )),
        Plantform::Douyu => Box::new(backend::douyu::StreamRoom::new(args.room_id, client)),
    };
    let url = room.get_url().await?;
    println!("{}", url);
    Ok(())
}
