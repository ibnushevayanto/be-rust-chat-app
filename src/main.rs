mod config;
mod modules;
mod utils;

use axum::{
    Json, Router,
    http::{StatusCode, Uri},
    routing::get,
};
use modules::auth::auth_routes;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = config::database::create_pool()
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        .route("/", get(index))
        .nest("/auth", auth_routes())
        .fallback(fallback_handler)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn fallback_handler(uri: Uri) -> (StatusCode, Json<Value>) {
    let response = json!({
        "status": false,
        "message": format!("Route not found: {}", uri)
    });

    (StatusCode::NOT_FOUND, Json(response))
}

async fn index() -> &'static str {
    "The project is running"
}
