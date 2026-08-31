use chrono::{Duration, Utc};
use motorguard_core::{AppError, Result};
use rand::Rng;
use sqlx::SqlitePool;
use tracing::{info, warn};

/// Generates, stores, and verifies 6-digit phone OTPs.
pub struct OtpService {
    pool: SqlitePool,
    expiry_minutes: i64,
    /// In dev mode, any code is accepted and actual SMS is skipped.
    dev_mode: bool,
}

impl OtpService {
    pub fn new(pool: SqlitePool, expiry_minutes: i64, dev_mode: bool) -> Self {
        Self {
            pool,
            expiry_minutes,
            dev_mode,
        }
    }

    /// Generate and store an OTP for the given phone number.
    /// Returns the plaintext code (to be sent via SMS).
    pub async fn generate(&self, phone: &str) -> Result<String> {
        let code = if self.dev_mode {
            // In dev mode, always use 123456 so you can test without SMS
            "123456".to_string()
        } else {
            let mut rng = rand::thread_rng();
            format!("{:06}", rng.gen_range(100_000..=999_999))
        };

        let code_hash = bcrypt::hash(&code, bcrypt::DEFAULT_COST)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("bcrypt hash failed")))?;

        let expires_at = Utc::now() + Duration::minutes(self.expiry_minutes);

        // Invalidate any existing OTP for this phone before inserting
        sqlx::query!(
            r#"DELETE FROM otp_codes WHERE phone = ?"#,
            phone
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        sqlx::query!(
            r#"INSERT INTO otp_codes (phone, code_hash, expires_at) VALUES (?, ?, ?)"#,
            phone,
            code_hash,
            expires_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if self.dev_mode {
            info!("DEV MODE: OTP for {} is {}", phone, code);
        }

        Ok(code)
    }

    /// Verify the submitted code against the stored hash.
    /// Consumes (deletes) the OTP on success.
    pub async fn verify(&self, phone: &str, code: &str) -> Result<()> {
        let row = sqlx::query!(
            r#"SELECT code_hash, expires_at, used FROM otp_codes WHERE phone = ? ORDER BY rowid DESC LIMIT 1"#,
            phone
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        let row = match row {
            Some(r) => r,
            None => {
                warn!("OTP verify: no code found for {}", phone);
                return Err(AppError::OtpInvalid);
            }
        };

        if row.used {
            return Err(AppError::OtpExpired);
        }

        let now = Utc::now().naive_utc();
        if row.expires_at < now {
            return Err(AppError::OtpExpired);
        }

        let valid = bcrypt::verify(code, &row.code_hash)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("bcrypt verify failed")))?;

        if !valid {
            return Err(AppError::OtpInvalid);
        }

        // Mark as used
        sqlx::query!(
            r#"UPDATE otp_codes SET used = 1 WHERE phone = ?"#,
            phone
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
