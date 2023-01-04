use axum::{
    extract::{Json, Path, Query, State},
    response::Redirect,
    routing::get,
};

use backend::GetUrl;
use clap::Parser;
use error::AppError;
use serde::{Deserialize, Serialize};
mod error;
#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value = "80")]
    port: u32,
    #[arg(short, long, default_value = "0.0.0.0")]
    addr: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct StreamRoomQuery {
    platform: String,
    room_id: String,
}

async fn post_url(
    State(client): State<reqwest::Client>,
    Json(StreamRoomQuery { platform, room_id }): Json<StreamRoomQuery>,
) -> Result<String, AppError> {
    let stream_room = backend::StreamRoom::new(platform.as_str(), room_id.as_str(), client);
    let url = stream_room.get_url().await?;
    Ok(url)
}

async fn get_url(
    State(client): State<reqwest::Client>,
    Query(StreamRoomQuery { platform, room_id }): Query<StreamRoomQuery>,
) -> Result<String, AppError> {
    let stream_room = backend::StreamRoom::new(platform.as_str(), room_id.as_str(), client);
    let url = stream_room.get_url().await?;
    Ok(url)
}

async fn redirect_url(
    State(client): State<reqwest::Client>,
    Path((platform, room_id)): Path<(String, String)>,
) -> Result<Redirect, AppError> {
    let stream_room = backend::StreamRoom::new(platform.as_str(), room_id.as_str(), client);
    let url = stream_room.get_url().await?;
    Ok(Redirect::temporary(url.as_str()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = reqwest::Client::builder().build()?;
    let app = axum::Router::new()
        .route("/", get(|| async { "hello from index" }))
        .route("/api/v1/stream/url", get(get_url).post(post_url))
        .route("/api/v1/stream/:platform/:room_id", get(redirect_url))
        .with_state(client);
    let addr = format!("{}:{}", args.addr, args.port).parse()?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    println!("{:?}", args);

    Ok(())
}