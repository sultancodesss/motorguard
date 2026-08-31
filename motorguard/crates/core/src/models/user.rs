use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::UserId;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: UserId,
    pub phone: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(phone: String) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            phone,
            name: None,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or("Rider")
    }
}

/// Lightweight stats summary attached to user profile responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserStats {
    pub total_rides: i64,
    pub total_miles: f64,
    pub safety_score: f64,
}
