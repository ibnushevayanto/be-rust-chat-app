use axum::Json;
use serde_json::{Value, json};

pub async fn register_process(payload: Json<Value>) {
    let username = payload.get("username").unwrap().as_str().unwrap();

    println!("{}", username);
}
