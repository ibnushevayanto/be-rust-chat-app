use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, time::Duration};

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let username = env::var("DB_USER").expect("DB_USER must be set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let host = env::var("DB_HOST").expect("DB_HOST must be set");
    let port = env::var("DB_PORT").expect("DB_PORT must be set");
    let database = env::var("DB_DATABASE").expect("DB_DATABASE must be set");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, database
    );

    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
}
