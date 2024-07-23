use std::env;
use dotenv::dotenv;

use axum::{response::IntoResponse, routing::get, Json, Router};

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";
    println!("<-GET 200 health_checker_handler");
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
    .route("/", get(health_checker_handler));

    let host = env::var("HOST").unwrap();
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    println!("🚀 Server started successfully on {}:{}", host, port);

    let listener = tokio::net::TcpListener::bind((host, port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
    