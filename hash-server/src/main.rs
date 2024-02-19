mod hash_ring;
mod server_location_service;

use axum::{
    extract::{Path, State}, http::StatusCode, response::Redirect, routing::{get, post}, Json, Router
};
use hash_ring::HashRing;
use server_location_service::ServerLocationService;
use std::{hash, sync::Arc};
use tokio;

async fn redirect(Path(key): Path<String>, State(location_service) : State<Arc<ServerLocationService>>) -> Result<Redirect, StatusCode> {
    if let Some(hostname) = location_service.get_hostname(&key).await {
        Ok(Redirect::temporary(&format!("https://{}", hostname) ))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {

    let hash_ring = HashRing::new(1 << 20, 16);
    let location_service = Arc::new(ServerLocationService::new(hash_ring));

    let loc = location_service.clone();
    tokio::spawn(async move {
        loc.listen_for_updates().await
    });

    let app = Router::new()
        .route("/serve/*key", get(redirect))
        .with_state(location_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
