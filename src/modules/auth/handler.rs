use super::dto::RegisterDTO;
use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use validator::Validate;

use super::service;

pub async fn login(State(pool): State<PgPool>, Json(payload): Json<Value>) -> Json<Value> {
    let username = payload.get("username").unwrap().as_str().unwrap();
    let password = payload.get("password").unwrap().as_str().unwrap();

    let response = json!(
        {
            "status": true,
            "message": format!("Login handler {}, {}", username, password)
        }
    );

    Json(response)
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterDTO>,
) -> (StatusCode, Json<Value>) {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": false,
                "message": "Validation error",
                "errors": errors
            })),
        );
    }

    let result = service::register_process(&pool, &payload).await;

    match result {
        Ok(value) => (StatusCode::OK, value),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": false,
                "message": error
            })),
        ),
    }
}
