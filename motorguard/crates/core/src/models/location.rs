use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::UserId;

/// A timestamped GPS coordinate with optional metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    /// Speed in mph.
    pub speed: f64,
    /// Heading in degrees (0 = north, 90 = east).
    pub heading: Option<f64>,
    /// GPS accuracy radius in metres (lower = better).
    pub accuracy: f64,
    pub timestamp: DateTime<Utc>,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64, speed: f64, accuracy: f64) -> Self {
        Self {
            latitude,
            longitude,
            altitude: None,
            speed,
            heading: None,
            accuracy,
            timestamp: Utc::now(),
        }
    }

    /// Check whether coordinates are within valid WGS84 bounds.
    pub fn is_valid_coordinates(&self) -> bool {
        self.latitude >= -90.0
            && self.latitude <= 90.0
            && self.longitude >= -180.0
            && self.longitude <= 180.0
    }

    /// Check whether the accuracy is acceptable for ride tracking.
    /// Returns false if accuracy radius exceeds the given threshold.
    pub fn is_accurate_enough(&self, threshold_metres: f64) -> bool {
        self.accuracy <= threshold_metres
    }
}

/// Stored user location for group sharing.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StoredLocation {
    pub user_id: UserId,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub accuracy: f64,
    pub recorded_at: DateTime<Utc>,
}
