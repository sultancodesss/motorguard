use chrono::{Duration, Utc};
use motorguard_core::{
    models::{Session, User},
    types::{SessionId, UserId},
    AppError, Result,
};
use motorguard_database::{SessionRepository, UserRepository};
use sqlx::SqlitePool;
use tracing::info;

use crate::{jwt::JwtService, otp::OtpService};

/// Tokens returned after successful OTP verification.
#[derive(Debug, Clone)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: User,
    pub is_new_user: bool,
}

/// High-level authentication service.
pub struct AuthService {
    pool: SqlitePool,
    jwt: JwtService,
    otp: OtpService,
    refresh_expiry_days: i64,
}

impl AuthService {
    pub fn new(
        pool: SqlitePool,
        jwt: JwtService,
        otp: OtpService,
        refresh_expiry_days: i64,
    ) -> Self {
        Self {
            pool,
            jwt,
            otp,
            refresh_expiry_days,
        }
    }

    /// Step 1: Request an OTP for the phone number.
    /// Returns the plaintext OTP (to be forwarded to SMS service).
    pub async fn request_otp(&self, phone: &str) -> Result<String> {
        validate_phone(phone)?;
        let code = self.otp.generate(phone).await?;
        Ok(code)
    }

    /// Step 2: Verify OTP, create or fetch user, return tokens.
    pub async fn verify_otp(&self, phone: &str, code: &str) -> Result<AuthTokens> {
        validate_phone(phone)?;
        self.otp.verify(phone, code).await?;

        let users = UserRepository::new(&self.pool);

        // Find existing user or create new one
        let (user, is_new) = match users.find_by_phone(phone).await? {
            Some(u) => (u, false),
            None => {
                let new_user = User::new(phone.to_string());
                let created = users.create(&new_user).await?;
                info!("New user registered: {}", created.id);
                (created, true)
            }
        };

        let tokens = self.create_tokens(user, is_new).await?;
        Ok(tokens)
    }

    /// Exchange a valid refresh token for a new access token.
    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<(String, i64)> {
        let claims = self.jwt.verify(refresh_token)?;
        if claims.typ != "refresh" {
            return Err(AppError::TokenMalformed);
        }
        let user_id = claims.user_id()?;
        let access_token = self.jwt.create_access_token(user_id)?;
        let expires_in = 24 * 3600; // 24h in seconds
        Ok((access_token, expires_in))
    }

    /// Invalidate the session associated with the given session id.
    pub async fn logout(&self, session_id: SessionId) -> Result<()> {
        let sessions = SessionRepository::new(&self.pool);
        sessions.delete(session_id).await?;
        Ok(())
    }

    // ── Private helpers ──────────────────────────────────────────────────────

    async fn create_tokens(&self, user: User, is_new_user: bool) -> Result<AuthTokens> {
        let access_token = self.jwt.create_access_token(user.id)?;
        let refresh_token = self.jwt.create_refresh_token(user.id)?;

        // Hash and store the refresh token
        let token_hash = bcrypt::hash(&refresh_token, 4)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("bcrypt hash failed")))?;

        let session = Session {
            id: SessionId::new(),
            user_id: user.id,
            refresh_token_hash: token_hash,
            expires_at: Utc::now() + Duration::days(self.refresh_expiry_days),
            created_at: Utc::now(),
        };

        let sessions = SessionRepository::new(&self.pool);
        sessions.create(&session).await?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
            expires_in: 24 * 3600,
            user,
            is_new_user,
        })
    }
}

/// Validate phone is in E.164 format: +[1-15 digits]
fn validate_phone(phone: &str) -> Result<()> {
    let re = regex::Regex::new(r"^\+[1-9]\d{6,14}$").unwrap();
    if !re.is_match(phone) {
        return Err(AppError::InvalidPhone);
    }
    Ok(())
}
