use motorguard_core::{
    models::{User, UserStats},
    types::UserId,
    AppError, Result,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct UserRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: UserId) -> Result<User> {
        let uuid_str = id.to_string();
        sqlx::query_as!(
            User,
            r#"
            SELECT id as "id: UserId", phone, name, avatar_url,
                   created_at as "created_at: _", updated_at as "updated_at: _"
            FROM users WHERE id = ?
            "#,
            uuid_str
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("User".to_string()),
            _ => AppError::Database(e.to_string()),
        })
    }

    pub async fn find_by_phone(&self, phone: &str) -> Result<Option<User>> {
        let row = sqlx::query!(
            r#"SELECT id, phone, name, avatar_url, created_at, updated_at FROM users WHERE phone = ?"#,
            phone
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(row.map(|r| User {
            id: UserId::from_uuid(Uuid::parse_str(&r.id).unwrap_or_default()),
            phone: r.phone,
            name: r.name,
            avatar_url: r.avatar_url,
            created_at: r.created_at.and_utc(),
            updated_at: r.updated_at.and_utc(),
        }))
    }

    pub async fn create(&self, user: &User) -> Result<User> {
        let id_str = user.id.to_string();
        sqlx::query!(
            r#"
            INSERT INTO users (id, phone, name, avatar_url, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            id_str,
            user.phone,
            user.name,
            user.avatar_url,
            user.created_at,
            user.updated_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(user.clone())
    }

    pub async fn update_profile(
        &self,
        id: UserId,
        name: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<User> {
        let id_str = id.to_string();
        let now = chrono::Utc::now();
        sqlx::query!(
            r#"UPDATE users SET name = ?, avatar_url = ?, updated_at = ? WHERE id = ?"#,
            name,
            avatar_url,
            now,
            id_str,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_by_id(id).await
    }

    pub async fn get_stats(&self, user_id: UserId) -> Result<UserStats> {
        let user_id_str = user_id.to_string();
        let row = sqlx::query!(
            r#"
            SELECT
                COUNT(*) as total_rides,
                COALESCE(SUM(distance_miles), 0.0) as total_miles,
                COALESCE(AVG(safety_score), 0.0) as avg_safety
            FROM rides
            WHERE user_id = ? AND status = 'completed'
            "#,
            user_id_str
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(UserStats {
            total_rides: row.total_rides,
            total_miles: row.total_miles.unwrap_or(0.0),
            safety_score: row.avg_safety.unwrap_or(0.0),
        })
    }
}
