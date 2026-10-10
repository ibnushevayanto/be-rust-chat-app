use axum::{
    Json, Router,
    http::{StatusCode, Uri},
    routing::get,
};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use std::env;

mod modules;
use modules::auth::auth_routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    println!("Berhasil terhubung ke database YugabyteDB!");

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
