use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{SosEventId, UserId, SosStatus, SosTrigger};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SosEvent {
    pub id: SosEventId,
    pub user_id: UserId,
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub trigger: SosTrigger,
    pub status: SosStatus,
    pub contacts_notified: i32,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolve_reason: Option<String>,
}

impl SosEvent {
    pub fn new(
        user_id: UserId,
        latitude: f64,
        longitude: f64,
        accuracy: f64,
        trigger: SosTrigger,
    ) -> Self {
        Self {
            id: SosEventId::new(),
            user_id,
            latitude,
            longitude,
            accuracy,
            trigger,
            status: SosStatus::Active,
            contacts_notified: 0,
            created_at: Utc::now(),
            resolved_at: None,
            resolve_reason: None,
        }
    }
}
