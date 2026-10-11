-- Buat tabel user_backup_settings (sesuai schema compac-api versi Rust)
CREATE TABLE IF NOT EXISTS user_backup_settings (
    user_id        UUID        PRIMARY KEY,
    backup_enabled BOOLEAN     NOT NULL DEFAULT FALSE,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_user_backup_settings_users FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Index untuk mempercepat lookup pengaturan backup pengguna
CREATE INDEX IF NOT EXISTS idx_user_backup_settings_user_id ON user_backup_settings(user_id);
