use axum::{
    extract::{Json, Path, Query, State},
    response::Redirect,
    routing::{get, post},
};

use backend::{GetUrl, GetUrls};
use clap::Parser;
use error::AppError;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::Level;

mod error;
#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value = "80")]
    port: u32,
    #[arg(short, long, default_value = "0.0.0.0")]
    addr: String,
    #[arg(short, long)]
    debug: bool,
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

async fn get_all_urls(
    State(client): State<reqwest::Client>,
    Json(StreamRoomQuery { platform, room_id }): Json<StreamRoomQuery>,
) -> Result<Json<Vec<String>>, AppError> {
    let stream_room = backend::StreamRoom::new(platform.as_str(), room_id.as_str(), client);
    let urls = stream_room.get_urls().await?;
    Ok(Json(urls))
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
    tracing_subscriber::fmt()
        .with_max_level(if args.debug {
            Level::DEBUG
        } else {
            Level::INFO
        })
        .init();
    let client = reqwest::Client::builder().build()?;
    let app = axum::Router::new()
        .route("/", get(|| async { "hello from index" }))
        .route("/api/v1/stream/url", get(get_url).post(post_url))
        .route("/api/v1/stream/all_urls", post(get_all_urls))
        .route("/api/v1/stream/:platform/:room_id", get(redirect_url))
        .layer(TraceLayer::new_for_http())
        .with_state(client);
    let addr = format!("{}:{}", args.addr, args.port).parse()?;
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
