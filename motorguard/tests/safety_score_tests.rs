/// Tests for the ride safety score algorithm.

use motorguard_core::{
    models::RidePoint,
    types::{RideId, RidePointId},
};
use motorguard_rides::safety_score::calculate_safety_score;
use motorguard_rides::statistics::RideStatistics;
use chrono::Utc;

fn make_point(speed: f64, secs_offset: i64, accuracy: f64) -> RidePoint {
    RidePoint {
        id:          RidePointId::new(),
        ride_id:     RideId::new(),
        latitude:    37.7749,
        longitude:   -122.4194,
        altitude:    None,
        speed,
        accuracy,
        recorded_at: Utc::now() + chrono::Duration::seconds(secs_offset),
    }
}

// ── Perfect ride ─────────────────────────────────────────────────────────────

#[test]
fn perfect_ride_scores_100() {
    // Smooth 40 mph throughout, excellent GPS
    let points: Vec<RidePoint> = (0..10)
        .map(|i| make_point(40.0, i * 30, 5.0))
        .collect();
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert_eq!(score, 100.0, "Smooth low-speed ride should score 100, got {score}");
}

// ── Empty ride ────────────────────────────────────────────────────────────────

#[test]
fn empty_points_scores_100() {
    let stats = RideStatistics::from_points(&[]);
    let score = calculate_safety_score(&[], &stats);
    assert_eq!(score, 100.0, "No data defaults to 100");
}

// ── High speed penalty ────────────────────────────────────────────────────────

#[test]
fn max_speed_90mph_deducts_points() {
    let points: Vec<RidePoint> = (0..5)
        .map(|i| make_point(90.0, i * 30, 5.0))
        .collect();
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert!(score < 100.0, "90 mph ride should score below 100, got {score}");
    assert!(score >= 98.0, "90 mph deduction should be small (~2pts), got {score}");
}

#[test]
fn max_speed_150mph_large_deduction() {
    let points: Vec<RidePoint> = (0..5)
        .map(|i| make_point(150.0, i * 30, 5.0))
        .collect();
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert!(score <= 86.0, "150 mph should lose ~14 pts, got {score}");
}

#[test]
fn max_speed_penalty_capped_at_15() {
    // 200 mph — penalty capped at 15 points
    let points: Vec<RidePoint> = (0..5)
        .map(|i| make_point(200.0, i * 30, 5.0))
        .collect();
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert!(score >= 85.0, "Speed penalty capped at 15 pts, so score >= 85, got {score}");
}

// ── Harsh braking ─────────────────────────────────────────────────────────────

#[test]
fn harsh_braking_deducts_points() {
    // Simulate a sudden stop: 60 mph → 0 mph in 2 seconds
    let mut points = Vec::new();
    points.push(make_point(60.0, 0,  5.0));
    points.push(make_point(0.0,  2,  5.0));  // -30 mph/s = harsh
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert!(score < 100.0, "Harsh brake should reduce score, got {score}");
}

#[test]
fn smooth_deceleration_no_penalty() {
    // Gradual slowdown over 60 seconds: 60→0 = 1 mph/s — acceptable
    let points: Vec<RidePoint> = (0..7)
        .map(|i| make_point(60.0 - (i as f64 * 10.0), i * 10, 5.0))
        .collect();
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert!(score >= 98.0, "Smooth deceleration should not lose many pts, got {score}");
}

// ── Poor GPS accuracy ─────────────────────────────────────────────────────────

#[test]
fn mostly_poor_accuracy_deducts_5pts() {
    // 80% of points have >30m accuracy
    let mut points: Vec<RidePoint> = (0..8)
        .map(|i| make_point(40.0, i * 30, 50.0))   // poor accuracy
        .collect();
    points.push(make_point(40.0, 8 * 30, 5.0));     // good
    points.push(make_point(40.0, 9 * 30, 5.0));     // good
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert!(score <= 95.0, "Mostly poor GPS should deduct 5pts, got {score}");
}

#[test]
fn score_never_below_zero() {
    // Worst possible ride — high speed + harsh braking + poor GPS
    let mut points = Vec::new();
    // High speed
    for i in 0..5 { points.push(make_point(200.0, i * 2, 100.0)); }
    // Harsh brakes
    points.push(make_point(200.0, 20, 100.0));
    points.push(make_point(0.0,   21, 100.0));
    points.push(make_point(200.0, 22, 100.0));
    points.push(make_point(0.0,   23, 100.0));
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert!(score >= 0.0, "Score cannot be negative, got {score}");
}

#[test]
fn score_never_above_100() {
    let points: Vec<RidePoint> = (0..5)
        .map(|i| make_point(30.0, i * 60, 2.0))
        .collect();
    let stats = RideStatistics::from_points(&points);
    let score = calculate_safety_score(&points, &stats);
    assert!(score <= 100.0, "Score cannot exceed 100, got {score}");
}

// ── Statistics ────────────────────────────────────────────────────────────────

#[test]
fn statistics_empty_points() {
    let stats = RideStatistics::from_points(&[]);
    assert_eq!(stats.distance_miles,   0.0);
    assert_eq!(stats.average_speed_mph, 0.0);
    assert_eq!(stats.max_speed_mph,     0.0);
    assert_eq!(stats.point_count,       0);
}

#[test]
fn statistics_single_point() {
    let points = vec![make_point(45.0, 0, 5.0)];
    let stats = RideStatistics::from_points(&points);
    assert_eq!(stats.max_speed_mph,      45.0);
    assert_eq!(stats.average_speed_mph,  45.0);
    assert_eq!(stats.distance_miles,     0.0);   // single point = no distance
}

#[test]
fn statistics_max_speed_is_peak() {
    let points = vec![
        make_point(30.0, 0,  5.0),
        make_point(75.0, 30, 5.0),
        make_point(40.0, 60, 5.0),
    ];
    let stats = RideStatistics::from_points(&points);
    assert_eq!(stats.max_speed_mph, 75.0);
}
