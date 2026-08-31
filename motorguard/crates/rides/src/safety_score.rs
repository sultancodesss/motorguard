use motorguard_core::models::RidePoint;
use crate::statistics::RideStatistics;

/// Calculate a safety score 0–100 based on ride behaviour.
///
/// Scoring:
/// - Start at 100
/// - Deduct for high max speeds (1 pt per 5 mph over 80, capped at 15)
/// - Deduct 2 pts per harsh acceleration/braking event (capped at 20)
/// - Deduct 5 pts if >30% of points have accuracy > 30m
pub fn calculate_safety_score(points: &[RidePoint], stats: &RideStatistics) -> f64 {
    if points.is_empty() {
        return 100.0;
    }

    let mut score = 100.0_f64;

    // ── Max speed penalty ────────────────────────────────────────────────────
    if stats.max_speed_mph > 80.0 {
        let excess = (stats.max_speed_mph - 80.0) / 5.0;
        score -= excess.min(15.0);
    }

    // ── Harsh braking / acceleration ─────────────────────────────────────────
    let mut harsh_events = 0u32;
    for window in points.windows(2) {
        let delta = (window[1].speed - window[0].speed).abs();
        let dt = (window[1].recorded_at - window[0].recorded_at)
            .num_seconds()
            .max(1) as f64;
        let rate = delta / dt; // mph per second
        if rate > 10.0 {
            harsh_events += 1;
        }
    }
    score -= (harsh_events as f64 * 2.0).min(20.0);

    // ── Poor GPS accuracy penalty ────────────────────────────────────────────
    let poor_count = points.iter().filter(|p| p.accuracy > 30.0).count();
    let poor_ratio = poor_count as f64 / points.len() as f64;
    if poor_ratio > 0.3 {
        score -= 5.0;
    }

    score.clamp(0.0, 100.0).round()
}

#[cfg(test)]
mod tests {
    use super::*;
    use motorguard_core::types::{RideId, RidePointId};
    use chrono::Utc;

    fn pt(speed: f64, secs: i64, acc: f64) -> RidePoint {
        RidePoint {
            id:          RidePointId::new(),
            ride_id:     RideId::new(),
            latitude:    37.7749,
            longitude:   -122.4194,
            altitude:    None,
            speed,
            accuracy:    acc,
            recorded_at: Utc::now() + chrono::Duration::seconds(secs),
        }
    }

    #[test]
    fn smooth_ride_scores_100() {
        let pts: Vec<_> = (0..8).map(|i| pt(40.0, i * 30, 5.0)).collect();
        let stats = RideStatistics::from_points(&pts);
        assert_eq!(calculate_safety_score(&pts, &stats), 100.0);
    }

    #[test]
    fn empty_scores_100() {
        let stats = RideStatistics::from_points(&[]);
        assert_eq!(calculate_safety_score(&[], &stats), 100.0);
    }

    #[test]
    fn score_bounded_0_to_100() {
        let pts: Vec<_> = (0..4).map(|i| pt(200.0, i, 100.0)).collect();
        let stats = RideStatistics::from_points(&pts);
        let score = calculate_safety_score(&pts, &stats);
        assert!((0.0..=100.0).contains(&score), "Out of range: {score}");
    }

    #[test]
    fn high_speed_deducts() {
        let pts: Vec<_> = (0..5).map(|i| pt(100.0, i * 30, 5.0)).collect();
        let stats = RideStatistics::from_points(&pts);
        assert!(calculate_safety_score(&pts, &stats) < 100.0);
    }
}
