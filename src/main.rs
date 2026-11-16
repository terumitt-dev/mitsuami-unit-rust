use axum::{Json, Router, routing::get};
use std::collections::HashMap;

async fn healthz() -> Json<HashMap<&'static str, &'static str>> {
    Json(HashMap::from([("status", "ok")]))
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let app = Router::new().route("/healthz", get(healthz));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
