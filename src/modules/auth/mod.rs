mod handler;
mod service;

use axum::{Router, routing::post};
use handler::{login, register};
use sqlx::PgPool;

pub fn auth_routes() -> Router<PgPool> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
