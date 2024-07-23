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
    let app = Router::new().route("/", get(health_checker_handler));

    println!("🚀 Server started successfully in port 8000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
