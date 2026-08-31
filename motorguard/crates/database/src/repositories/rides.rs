use motorguard_core::{
    models::{Ride, RidePoint},
    types::{RideId, RidePointId, UserId, RideStatus},
    AppError, Result,
};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct RideRepository<'a> {
    pool: &'a SqlitePool,
}

impl<'a> RideRepository<'a> {
    pub fn new(pool: &'a SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, ride: &Ride) -> Result<Ride> {
        let id_str = ride.id.to_string();
        let user_id_str = ride.user_id.to_string();
        let status_str = ride.status.to_string();
        sqlx::query!(
            r#"
            INSERT INTO rides (id, user_id, name, status, distance_miles, duration_seconds,
                               average_speed_mph, max_speed_mph, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id_str,
            user_id_str,
            ride.name,
            status_str,
            ride.distance_miles,
            ride.duration_seconds,
            ride.average_speed_mph,
            ride.max_speed_mph,
            ride.created_at,
            ride.updated_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(ride.clone())
    }

    pub async fn find_by_id(&self, id: RideId) -> Result<Ride> {
        let id_str = id.to_string();
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, name, status, started_at, ended_at,
                   distance_miles, duration_seconds, average_speed_mph, max_speed_mph,
                   safety_score, route_summary, created_at, updated_at
            FROM rides WHERE id = ?
            "#,
            id_str
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Ride".to_string()),
            _ => AppError::Database(e.to_string()),
        })?;

        Ok(Ride {
            id: RideId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
            user_id: UserId::from_uuid(Uuid::parse_str(&row.user_id).unwrap_or_default()),
            name: row.name,
            status: parse_ride_status(&row.status),
            started_at: row.started_at.map(|dt| dt.and_utc()),
            ended_at: row.ended_at.map(|dt| dt.and_utc()),
            distance_miles: row.distance_miles,
            duration_seconds: row.duration_seconds,
            average_speed_mph: row.average_speed_mph,
            max_speed_mph: row.max_speed_mph,
            safety_score: row.safety_score,
            route_summary: row.route_summary,
            created_at: row.created_at.and_utc(),
            updated_at: row.updated_at.and_utc(),
        })
    }

    pub async fn list_by_user(
        &self,
        user_id: UserId,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Ride>> {
        let user_id_str = user_id.to_string();
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, name, status, started_at, ended_at,
                   distance_miles, duration_seconds, average_speed_mph, max_speed_mph,
                   safety_score, route_summary, created_at, updated_at
            FROM rides
            WHERE user_id = ? AND status = 'completed'
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id_str,
            limit,
            offset
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| Ride {
                id: RideId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
                user_id: UserId::from_uuid(Uuid::parse_str(&row.user_id).unwrap_or_default()),
                name: row.name,
                status: parse_ride_status(&row.status),
                started_at: row.started_at.map(|dt| dt.and_utc()),
                ended_at: row.ended_at.map(|dt| dt.and_utc()),
                distance_miles: row.distance_miles,
                duration_seconds: row.duration_seconds,
                average_speed_mph: row.average_speed_mph,
                max_speed_mph: row.max_speed_mph,
                safety_score: row.safety_score,
                route_summary: row.route_summary,
                created_at: row.created_at.and_utc(),
                updated_at: row.updated_at.and_utc(),
            })
            .collect())
    }

    pub async fn set_status(&self, id: RideId, status: RideStatus) -> Result<()> {
        let id_str = id.to_string();
        let status_str = status.to_string();
        let now = Utc::now();
        sqlx::query!(
            r#"UPDATE rides SET status = ?, updated_at = ? WHERE id = ?"#,
            status_str,
            now,
            id_str,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn start_ride(&self, id: RideId) -> Result<()> {
        let id_str = id.to_string();
        let now = Utc::now();
        sqlx::query!(
            r#"UPDATE rides SET status = 'active', started_at = ?, updated_at = ? WHERE id = ?"#,
            now,
            now,
            id_str,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn finish_ride(
        &self,
        id: RideId,
        distance_miles: f64,
        duration_seconds: i64,
        average_speed_mph: f64,
        max_speed_mph: f64,
        safety_score: f64,
    ) -> Result<()> {
        let id_str = id.to_string();
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE rides
            SET status = 'completed', ended_at = ?, updated_at = ?,
                distance_miles = ?, duration_seconds = ?,
                average_speed_mph = ?, max_speed_mph = ?, safety_score = ?
            WHERE id = ?
            "#,
            now,
            now,
            distance_miles,
            duration_seconds,
            average_speed_mph,
            max_speed_mph,
            safety_score,
            id_str,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn insert_point(&self, point: &RidePoint) -> Result<()> {
        let id_str = point.id.to_string();
        let ride_id_str = point.ride_id.to_string();
        sqlx::query!(
            r#"
            INSERT INTO ride_points (id, ride_id, latitude, longitude, altitude, speed, accuracy, recorded_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id_str,
            ride_id_str,
            point.latitude,
            point.longitude,
            point.altitude,
            point.speed,
            point.accuracy,
            point.recorded_at,
        )
        .execute(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn get_points(&self, ride_id: RideId) -> Result<Vec<RidePoint>> {
        let ride_id_str = ride_id.to_string();
        let rows = sqlx::query!(
            r#"
            SELECT id, ride_id, latitude, longitude, altitude, speed, accuracy, recorded_at
            FROM ride_points WHERE ride_id = ? ORDER BY recorded_at ASC
            "#,
            ride_id_str
        )
        .fetch_all(self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| RidePoint {
                id: RidePointId::from_uuid(Uuid::parse_str(&row.id).unwrap_or_default()),
                ride_id: RideId::from_uuid(Uuid::parse_str(&row.ride_id).unwrap_or_default()),
                latitude: row.latitude,
                longitude: row.longitude,
                altitude: row.altitude,
                speed: row.speed,
                accuracy: row.accuracy,
                recorded_at: row.recorded_at.and_utc(),
            })
            .collect())
    }
}

fn parse_ride_status(s: &str) -> RideStatus {
    match s {
        "active" => RideStatus::Active,
        "paused" => RideStatus::Paused,
        "completed" => RideStatus::Completed,
        "cancelled" => RideStatus::Cancelled,
        _ => RideStatus::Pending,
    }
}
