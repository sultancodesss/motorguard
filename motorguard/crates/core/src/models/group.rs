use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{GroupId, UserId, GroupRole};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: UserId,
    pub invite_code: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Group {
    pub fn new(name: String, description: Option<String>, owner_id: UserId) -> Self {
        let now = Utc::now();
        Self {
            id: GroupId::new(),
            name,
            description,
            owner_id,
            invite_code: generate_invite_code(),
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Membership record linking a user to a group.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GroupMember {
    pub group_id: GroupId,
    pub user_id: UserId,
    pub role: GroupRole,
    pub joined_at: DateTime<Utc>,
}

/// Generates a random 6-character uppercase invite code.
fn generate_invite_code() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    // Simple deterministic code — in production use rand crate
    format!("{:06X}", seed % 0xFF_FF_FF)
}
