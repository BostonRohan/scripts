use axum::{routing::post, Router};
mod wix_oauth;
use crate::wix_oauth::webhook;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(webhook));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
