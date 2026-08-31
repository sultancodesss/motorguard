use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{SessionId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    /// bcrypt hash of the refresh token — raw token is never stored.
    pub refresh_token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
