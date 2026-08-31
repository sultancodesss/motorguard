use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{EmergencyContactId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmergencyContact {
    pub id: EmergencyContactId,
    pub user_id: UserId,
    pub name: String,
    pub phone: String,
    pub relationship: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl EmergencyContact {
    pub fn new(user_id: UserId, name: String, phone: String, relationship: Option<String>) -> Self {
        Self {
            id: EmergencyContactId::new(),
            user_id,
            name,
            phone,
            relationship,
            created_at: Utc::now(),
        }
    }
}
