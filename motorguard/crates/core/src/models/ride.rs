use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{RideId, RidePointId, UserId, RideStatus};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ride {
    pub id: RideId,
    pub user_id: UserId,
    pub name: Option<String>,
    pub status: RideStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    /// Total distance in miles.
    pub distance_miles: f64,
    /// Duration in seconds.
    pub duration_seconds: i64,
    pub average_speed_mph: f64,
    pub max_speed_mph: f64,
    /// Safety score 0–100.
    pub safety_score: Option<f64>,
    pub route_summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Ride {
    pub fn new(user_id: UserId, name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: RideId::new(),
            user_id,
            name,
            status: RideStatus::Pending,
            started_at: None,
            ended_at: None,
            distance_miles: 0.0,
            duration_seconds: 0,
            average_speed_mph: 0.0,
            max_speed_mph: 0.0,
            safety_score: None,
            route_summary: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Duration formatted as human-readable string (e.g. "28 min" or "1h 45m").
    pub fn duration_display(&self) -> String {
        let total = self.duration_seconds;
        let hours = total / 3600;
        let minutes = (total % 3600) / 60;
        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{} min", minutes)
        }
    }
}

/// A single GPS data point recorded during an active ride.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RidePoint {
    pub id: RidePointId,
    pub ride_id: RideId,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    /// Speed in mph.
    pub speed: f64,
    /// GPS accuracy radius in metres.
    pub accuracy: f64,
    pub recorded_at: DateTime<Utc>,
}
