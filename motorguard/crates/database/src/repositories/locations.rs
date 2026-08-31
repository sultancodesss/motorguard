use motorguard_core::{
    models::location::StoredLocation,
    types::UserId,
    AppError, Result,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct LocationRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> LocationRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn upsert(&self, loc: &StoredLocation) -> Result<()> {
        let user_id_str = loc.user_id.to_string();
        sqlx::query!(
            r#"
            INSERT INTO locations (user_id, latitude, longitude, speed, accuracy, recorded_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(user_id) DO UPDATE SET
                latitude = excluded.latitude,
                longitude = excluded.longitude,
                speed = excluded.speed,
                accuracy = excluded.accuracy,
                recorded_at = excluded.recorded_at
            "#,
            user_id_str,
            loc.latitude,
            loc.longitude,
            loc.speed,
            loc.accuracy,
            loc.recorded_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn get_group_locations(&self, user_ids: &[String]) -> Result<Vec<StoredLocation>> {
        if user_ids.is_empty() {
            return Ok(vec![]);
        }
        // SQLx doesn't support dynamic IN lists natively with macros; use query_builder
        let placeholders = user_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let sql = format!(
            "SELECT user_id, latitude, longitude, speed, accuracy, recorded_at FROM locations WHERE user_id IN ({})",
            placeholders
        );
        let mut q = sqlx::query_as::<_, (String, f64, f64, f64, f64, chrono::NaiveDateTime)>(&sql);
        for id in user_ids {
            q = q.bind(id);
        }
        let rows = q
            .fetch_all(self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|(uid, lat, lon, speed, acc, ts)| StoredLocation {
                user_id: UserId::from_uuid(Uuid::parse_str(&uid).unwrap_or_default()),
                latitude: lat,
                longitude: lon,
                speed,
                accuracy: acc,
                recorded_at: ts.and_utc(),
            })
            .collect())
    }
}
