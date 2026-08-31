use motorguard_core::models::RidePoint;
use motorguard_location::distance::haversine_miles;

/// Aggregated statistics computed from a slice of ride points.
#[derive(Debug, Default)]
pub struct RideStatistics {
    pub distance_miles: f64,
    pub average_speed_mph: f64,
    pub max_speed_mph: f64,
    pub point_count: usize,
}

impl RideStatistics {
    pub fn from_points(points: &[RidePoint]) -> Self {
        if points.is_empty() {
            return Self::default();
        }

        let point_count = points.len();
        let mut distance = 0.0f64;
        let mut max_speed = 0.0f64;
        let mut speed_sum = 0.0f64;

        for i in 0..points.len() {
            let p = &points[i];
            if p.speed > max_speed {
                max_speed = p.speed;
            }
            speed_sum += p.speed;

            if i > 0 {
                let prev = &points[i - 1];
                distance += haversine_miles(
                    prev.latitude,
                    prev.longitude,
                    p.latitude,
                    p.longitude,
                );
            }
        }

        let avg_speed = if point_count > 0 {
            speed_sum / point_count as f64
        } else {
            0.0
        };

        Self {
            distance_miles: distance,
            average_speed_mph: avg_speed,
            max_speed_mph: max_speed,
            point_count,
        }
    }
}
