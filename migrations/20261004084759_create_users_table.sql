-- Extension untuk UUID (jika belum aktif)
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Buat tabel users (sesuai schema compac-api versi Rust)
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) NOT NULL,
    nama VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    email_canonical VARCHAR(255),
    password_hash VARCHAR(255) NOT NULL,
    salt BYTEA NOT NULL DEFAULT '\x'::bytea,
    phone_number_hash VARCHAR(64),
    organization VARCHAR(255),
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_verified BOOLEAN NOT NULL DEFAULT false,
    avatar_url TEXT,
    bio TEXT,
    pin_hash VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT users_username_key UNIQUE (username),
    CONSTRAINT users_email_canonical_key UNIQUE (email_canonical)
);

-- Index untuk mempercepat pencarian login dan registrasi
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email_canonical ON users(email_canonical);
CREATE INDEX IF NOT EXISTS idx_users_phone_number_hash ON users(phone_number_hash);
