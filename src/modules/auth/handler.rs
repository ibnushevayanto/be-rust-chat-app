use axum::Json;
use serde_json::{Value, json};

use super::service;

pub async fn login(Json(payload): Json<Value>) -> Json<Value> {
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

pub async fn register(Json(payload): Json<Value>) -> Json<Value> {
    let username = payload.get("username").unwrap().as_str().unwrap();
    let password = payload.get("password").unwrap().as_str().unwrap();

    service::register_process(Json(json!({
        "username": username,
        "password": password
    })))
    .await;

    let response = json!({
        "status": true,
        "message": format!("Register handler {}, {}", username, password)
    });

    Json(response)
}
