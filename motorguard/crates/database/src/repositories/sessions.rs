use motorguard_core::{
    models::Session,
    types::{SessionId, UserId},
    AppError, Result,
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct SessionRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> SessionRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, session: &Session) -> Result<Session> {
        let id_str = session.id.to_string();
        let user_id_str = session.user_id.to_string();
        sqlx::query!(
            r#"
            INSERT INTO sessions (id, user_id, refresh_token_hash, expires_at, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
            id_str,
            user_id_str,
            session.refresh_token_hash,
            session.expires_at,
            session.created_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(session.clone())
    }

    pub async fn find_by_id(&self, id: SessionId) -> Result<Session> {
        let id_str = id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, user_id, refresh_token_hash, expires_at, created_at FROM sessions WHERE id = ?"#,
            id_str
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Session".to_string()),
            _ => AppError::Database(e.to_string()),
        })?;

        Ok(Session {
            id: SessionId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
            user_id: UserId::from_uuid(Uuid::parse_str(&row.user_id).unwrap_or_default()),
            refresh_token_hash: row.refresh_token_hash,
            expires_at: row.expires_at.and_utc(),
            created_at: row.created_at.and_utc(),
        })
    }

    pub async fn delete(&self, id: SessionId) -> Result<()> {
        let id_str = id.to_string();
        sqlx::query!(r#"DELETE FROM sessions WHERE id = ?"#, id_str)
            .execute(self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn delete_expired(&self) -> Result<u64> {
        let now = Utc::now();
        let result = sqlx::query!(r#"DELETE FROM sessions WHERE expires_at < ?"#, now)
            .execute(self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(result.rows_affected())
    }
}
