use motorguard_core::{
    models::SosEvent,
    types::{SosEventId, UserId, SosStatus, SosTrigger},
    AppError, Result,
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct SosEventRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SosEventRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, event: &SosEvent) -> Result<SosEvent> {
        let id_str = event.id.to_string();
        let user_id_str = event.user_id.to_string();
        let trigger_str = format!("{:?}", event.trigger).to_lowercase();
        let status_str = format!("{:?}", event.status).to_lowercase();
        sqlx::query!(
            r#"
            INSERT INTO sos_events (id, user_id, latitude, longitude, accuracy, trigger, status, contacts_notified, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id_str,
            user_id_str,
            event.latitude,
            event.longitude,
            event.accuracy,
            trigger_str,
            status_str,
            event.contacts_notified,
            event.created_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(event.clone())
    }

    pub async fn find_by_id(&self, id: SosEventId) -> Result<SosEvent> {
        let id_str = id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, user_id, latitude, longitude, accuracy, trigger, status, contacts_notified, created_at, resolved_at, resolve_reason
               FROM sos_events WHERE id = ?"#,
            id_str
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::SosNotFound,
            _ => AppError::Database(e.to_string()),
        })?;

        Ok(SosEvent {
            id: SosEventId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
            user_id: UserId::from_uuid(Uuid::parse_str(&row.user_id).unwrap_or_default()),
            latitude: row.latitude,
            longitude: row.longitude,
            accuracy: row.accuracy,
            trigger: parse_trigger(&row.trigger),
            status: parse_sos_status(&row.status),
            contacts_notified: row.contacts_notified as i32,
            created_at: row.created_at.and_utc(),
            resolved_at: row.resolved_at.map(|dt| dt.and_utc()),
            resolve_reason: row.resolve_reason,
        })
    }

    pub async fn resolve(&self, id: SosEventId, reason: &str) -> Result<()> {
        let id_str = id.to_string();
        let now = Utc::now();
        sqlx::query!(
            r#"UPDATE sos_events SET status = 'resolved', resolved_at = ?, resolve_reason = ? WHERE id = ?"#,
            now,
            reason,
            id_str,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn update_contacts_notified(&self, id: SosEventId, count: i32) -> Result<()> {
        let id_str = id.to_string();
        sqlx::query!(
            r#"UPDATE sos_events SET contacts_notified = ? WHERE id = ?"#,
            count,
            id_str,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

fn parse_trigger(s: &str) -> SosTrigger {
    match s {
        "crash_detection" => SosTrigger::CrashDetection,
        _ => SosTrigger::Manual,
    }
}

fn parse_sos_status(s: &str) -> SosStatus {
    match s {
        "resolved" => SosStatus::Resolved,
        "false_alarm" => SosStatus::FalseAlarm,
        _ => SosStatus::Active,
    }
}
