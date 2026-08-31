use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{NotificationId, UserId, NotificationKind};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: NotificationId,
    pub user_id: UserId,
    pub kind: NotificationKind,
    pub title: String,
    pub body: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(
        user_id: UserId,
        kind: NotificationKind,
        title: String,
        body: String,
    ) -> Self {
        Self {
            id: NotificationId::new(),
            user_id,
            kind,
            title,
            body,
            is_read: false,
            created_at: Utc::now(),
        }
    }
}
