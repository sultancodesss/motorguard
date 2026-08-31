use motorguard_core::{
    models::Notification,
    types::{NotificationId, UserId, NotificationKind},
    AppError, Result,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct NotificationRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> NotificationRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, notif: &Notification) -> Result<Notification> {
        let id_str = notif.id.to_string();
        let user_id_str = notif.user_id.to_string();
        let kind_str = format!("{:?}", notif.kind).to_lowercase();
        sqlx::query!(
            r#"INSERT INTO notifications (id, user_id, kind, title, body, is_read, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            id_str,
            user_id_str,
            kind_str,
            notif.title,
            notif.body,
            notif.is_read,
            notif.created_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(notif.clone())
    }

    pub async fn list_by_user(&self, user_id: UserId, limit: i64) -> Result<Vec<Notification>> {
        let user_id_str = user_id.to_string();
        let rows = sqlx::query!(
            r#"SELECT id, user_id, kind, title, body, is_read, created_at FROM notifications WHERE user_id = ? ORDER BY created_at DESC LIMIT ?"#,
            user_id_str,
            limit
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| Notification {
                id: NotificationId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
                user_id: UserId::from_uuid(Uuid::parse_str(&row.user_id).unwrap_or_default()),
                kind: parse_kind(&row.kind),
                title: row.title,
                body: row.body,
                is_read: row.is_read,
                created_at: row.created_at.and_utc(),
            })
            .collect())
    }

    pub async fn mark_read(&self, id: NotificationId) -> Result<()> {
        let id_str = id.to_string();
        sqlx::query!(r#"UPDATE notifications SET is_read = 1 WHERE id = ?"#, id_str)
            .execute(self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

fn parse_kind(s: &str) -> NotificationKind {
    match s {
        "sos" => NotificationKind::Sos,
        "group_invite" => NotificationKind::GroupInvite,
        "ride_complete" => NotificationKind::RideComplete,
        _ => NotificationKind::SystemAlert,
    }
}
