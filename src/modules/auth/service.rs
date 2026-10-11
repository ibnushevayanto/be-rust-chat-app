use super::dto::RegisterDTO;
use crate::utils::email::get_canonical_email;
use axum::Json;
use chrono;
use pbkdf2::pbkdf2_hmac;
use rand::Rng;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

pub async fn register_process(pool: &PgPool, payload: &RegisterDTO) -> Result<Json<Value>, String> {
    let email_canonical = get_canonical_email(&payload.email);

    let is_email_unique = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email_canonical = $1)",
    )
    .bind(&email_canonical)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e));

    if is_email_unique? {
        return Err("Email is already taken".to_string());
    }

    let is_username_unqiue =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(&payload.username)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Database error: {}", e));

    if is_username_unqiue? {
        return Err("Username is already taken".to_string());
    }

    let mut salt = [0u8; 16];
    rand::rng().fill_bytes(&mut salt);

    let phone_hash = {
        let mut out = [0u8; 32];
        let phone_bytes = payload.phone.as_bytes();
        pbkdf2_hmac::<Sha256>(phone_bytes, &salt, 10_000, &mut out);
        hex::encode(out)
    };

    let is_phone_number_unique = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE phone_number_hash = $1)",
    )
    .bind(&phone_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Database error: {}", e));

    if is_phone_number_unique? {
        return Err("Phone number is already taken".to_string());
    }

    let password_hash = bcrypt::hash(&payload.password, 12).map_err(|e| format!("{}", e))?;
    let user_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    let otp_num = (rand::random::<u32>() % 900_000) + 100_000;
    let otp_code = format!("{:06}", otp_num);
    let otp_hash = hex::encode(Sha256::digest(otp_code.as_bytes()));
    let otp_expires_at = now + chrono::Duration::minutes(5);

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| format!("Failed to start transaction: {e}"))?;

    sqlx::query(
        "INSERT INTO users (
        id, username, nama, email, email_canonical, password_hash, 
        salt, phone_number_hash, organization, role, is_active, is_verified, 
        created_at, updated_at
    ) VALUES ($1::uuid, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
    )
    .bind(&user_id)
    .bind(&payload.username)
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&email_canonical)
    .bind(&password_hash)
    .bind(&salt[..])
    .bind(&phone_hash)
    .bind(&payload.organization)
    .bind("user")
    .bind(true)
    .bind(false)
    .bind(now)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to insert user: {e}"))?;

    let device_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO devices (
        id, user_id, device_type, device_name, device_id, 
        created_at, updated_at, device_location
    ) VALUES ($1::uuid, $2::uuid, $3, $4, $5, $6, $7, $8)",
    )
    .bind(&device_id)
    .bind(&user_id)
    .bind(&payload.device_type)
    .bind(&payload.device_name)
    .bind(&device_id)
    .bind(now)
    .bind(now)
    .bind(&payload.device_location)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to insert device: {e}"))?;

    sqlx::query(
        "INSERT INTO user_backup_settings (user_id, backup_enabled, updated_at) 
     VALUES ($1::uuid, $2, $3) 
     ON CONFLICT (user_id) DO NOTHING",
    )
    .bind(&user_id)
    .bind(&true)
    .bind(&now)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to insert user backup settings: {e}"))?;

    sqlx::query(
        "INSERT INTO otp_verifications (
        email, phone_number, otp_hash, expires_at, is_used, created_at, failed_attempts
    ) VALUES ($1, $2, $3, $4, false, $5, 0)",
    )
    .bind(&payload.email)
    .bind(&phone_hash)
    .bind(&otp_hash)
    .bind(otp_expires_at)
    .bind(now)
    .execute(&mut *tx)
    .await
    .map_err(|e| format!("Failed to save OTP: {e}"))?;

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {e}"))?;

    println!("==================================================");
    println!("📩 [MOCK EMAIL] Mengirim ke: {}", payload.email);
    println!("🔑 KODE VERIFIKASI (OTP): {}", otp_code);
    println!("⏰ Berlaku selama: 3 Menit");
    println!("==================================================");

    Ok(Json(json!({
        "status": true,
        "message": format!("Register success"),
    })))
}
