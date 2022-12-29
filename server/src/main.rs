use axum::{
    extract::{Query, State},
    routing::get,
};

use backend::GetUrl;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value = "80")]
    port: u32,
    #[arg(short, long, default_value = "0.0.0.0")]
    addr: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct StreamRoomQuery {
    plantform: Plantform,
    room_id: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
enum Plantform {
    Douyu,
    Bilibili,
    Huya,
}

async fn get_url(
    Query(StreamRoomQuery { plantform, room_id }): Query<StreamRoomQuery>,
    State(client): State<reqwest::Client>,
) -> String {
    let stream_room: Box<dyn GetUrl + Send + Sync> = match plantform {
        Plantform::Douyu => Box::new(backend::douyu::StreamRoom::new(room_id, client)),
        Plantform::Bilibili => Box::new(backend::bilibili::StreamRoom::new(room_id, client, 10000)),
        Plantform::Huya => Box::new(backend::huya::StreamRoom::new(room_id, client)),
    };
    let url = stream_room.get_url().await.unwrap();
    url
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::builder().build()?;
    let app = axum::Router::new()
        .route("/", get(|| async { "hello from index" }))
        .route("/api/v1/stream/url", get(get_url))
        .with_state(client);
    let addr = format!("{}:{}", args.addr, args.port).parse()?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    println!("{:?}", args);

    Ok(())
}
