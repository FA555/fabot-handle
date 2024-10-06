mod constant;
mod error;
mod game;
mod model;
mod route;
mod dict;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(route::ping))
        .route("/try_get_pinyin", get(route::try_get_pinyin))
        .route("/start", post(route::start))
        .route("/attempt", post(route::attempt));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:55551")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
