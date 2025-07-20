use axum::{
    routing::{get, post},
    Json, Router,
    serve,
};
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/echo", post(echo))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();  // ✅ replaces hyper::Server
}

async fn root() -> &'static str {
    "Hello from Rust with Axum!"
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    msg: String,
}

async fn echo(Json(payload): Json<Message>) -> Json<Message> {
    Json(payload)
}

