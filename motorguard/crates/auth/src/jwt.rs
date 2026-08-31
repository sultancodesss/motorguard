use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use motorguard_core::{types::UserId, AppError, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT claims embedded in every access token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — stringified UserId.
    pub sub: String,
    /// Issued at (Unix timestamp).
    pub iat: i64,
    /// Expiry (Unix timestamp).
    pub exp: i64,
    /// Token type — "access" or "refresh".
    pub typ: String,
}

impl Claims {
    pub fn user_id(&self) -> Result<UserId> {
        Uuid::parse_str(&self.sub)
            .map(UserId::from_uuid)
            .map_err(|_| AppError::TokenMalformed)
    }

    pub fn is_access_token(&self) -> bool {
        self.typ == "access"
    }
}

/// Thin wrapper around jsonwebtoken that encodes/decodes with our secret.
#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_expiry_hours: i64,
    refresh_expiry_days: i64,
}

impl JwtService {
    pub fn new(secret: &str, access_expiry_hours: i64, refresh_expiry_days: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_expiry_hours,
            refresh_expiry_days,
        }
    }

    /// Create a signed access token for the given user.
    pub fn create_access_token(&self, user_id: UserId) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.access_expiry_hours);
        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            typ: "access".to_string(),
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("Failed to sign access token")))
    }

    /// Create a signed refresh token for the given user.
    pub fn create_refresh_token(&self, user_id: UserId) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::days(self.refresh_expiry_days);
        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            typ: "refresh".to_string(),
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| AppError::Internal(anyhow::anyhow!("Failed to sign refresh token")))
    }

    /// Validate and decode any token, returning its claims.
    pub fn verify(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| {
                use jsonwebtoken::errors::ErrorKind;
                match e.kind() {
                    ErrorKind::ExpiredSignature => AppError::SessionExpired,
                    _ => AppError::TokenMalformed,
                }
            })
    }

    /// Verify and assert the token is an access token.
    pub fn verify_access_token(&self, token: &str) -> Result<Claims> {
        let claims = self.verify(token)?;
        if !claims.is_access_token() {
            return Err(AppError::TokenMalformed);
        }
        Ok(claims)
    }
}
