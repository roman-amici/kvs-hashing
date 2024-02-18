use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio;

async fn redirect(Path(key): Path<String>) -> Redirect {
    println!("{}", key);
    Redirect::temporary("https://google.com")
}

#[tokio::main]
async fn main() {
    let state = Arc::new("data".to_string());

    let app = Router::new()
        .route("/serve/*key", get(redirect))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
