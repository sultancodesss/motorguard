/// Integration-level tests for the location crate.
/// Run with: cargo test --test location_tests

use motorguard_location::distance::{haversine_miles, haversine_km, speed_mph, mph_to_kmh, kmh_to_mph};
use motorguard_core::models::Location;

// ── Haversine ────────────────────────────────────────────────────────────────

#[test]
fn haversine_zero_distance() {
    let d = haversine_miles(37.7749, -122.4194, 37.7749, -122.4194);
    assert!(d < 0.0001, "Same point should give ~0 miles, got {d}");
}

#[test]
fn haversine_nyc_to_la_approx() {
    // NYC → LA is roughly 2450 miles (great-circle)
    let d = haversine_miles(40.7128, -74.0060, 34.0522, -118.2437);
    assert!((d - 2450.0).abs() < 60.0, "NYC→LA expected ~2450 mi, got {d}");
}

#[test]
fn haversine_short_distance() {
    // ~1 mile apart in San Francisco
    let d = haversine_miles(37.7749, -122.4194, 37.7893, -122.4194);
    assert!((d - 0.99).abs() < 0.15, "Expected ~1 mile, got {d}");
}

#[test]
fn haversine_km_vs_miles_ratio() {
    let lat1 = 48.8566;
    let lon1 = 2.3522;
    let lat2 = 51.5074;
    let lon2 = -0.1278;
    let miles = haversine_miles(lat1, lon1, lat2, lon2);
    let km    = haversine_km(lat1, lon1, lat2, lon2);
    let ratio = km / miles;
    assert!((ratio - 1.60934).abs() < 0.01, "km/miles ratio should be ~1.609, got {ratio}");
}

// ── Speed ────────────────────────────────────────────────────────────────────

#[test]
fn speed_mph_one_hour() {
    let s = speed_mph(60.0, 3600.0);
    assert!((s - 60.0).abs() < 0.001, "Expected 60 mph, got {s}");
}

#[test]
fn speed_mph_zero_elapsed() {
    let s = speed_mph(10.0, 0.0);
    assert_eq!(s, 0.0, "Zero elapsed time should give 0 mph");
}

#[test]
fn speed_mph_half_hour() {
    let s = speed_mph(25.0, 1800.0);
    assert!((s - 50.0).abs() < 0.001, "Expected 50 mph, got {s}");
}

// ── Unit conversions ─────────────────────────────────────────────────────────

#[test]
fn mph_to_kmh_60() {
    let k = mph_to_kmh(60.0);
    assert!((k - 96.56).abs() < 0.1, "60 mph → ~96.56 km/h, got {k}");
}

#[test]
fn kmh_to_mph_100() {
    let m = kmh_to_mph(100.0);
    assert!((m - 62.14).abs() < 0.1, "100 km/h → ~62.14 mph, got {m}");
}

#[test]
fn round_trip_conversion() {
    let original = 75.0_f64;
    let converted = mph_to_kmh(original);
    let back = kmh_to_mph(converted);
    assert!((back - original).abs() < 0.001, "Round-trip should be lossless, got {back}");
}

// ── Location validation ───────────────────────────────────────────────────────

#[test]
fn valid_coordinates_accepted() {
    let loc = Location::new(37.7749, -122.4194, 35.0, 8.0);
    assert!(loc.is_valid_coordinates());
}

#[test]
fn north_pole_valid() {
    let loc = Location::new(90.0, 0.0, 0.0, 5.0);
    assert!(loc.is_valid_coordinates());
}

#[test]
fn south_pole_valid() {
    let loc = Location::new(-90.0, 0.0, 0.0, 5.0);
    assert!(loc.is_valid_coordinates());
}

#[test]
fn latitude_out_of_range_rejected() {
    let loc = Location::new(91.0, 0.0, 0.0, 5.0);
    assert!(!loc.is_valid_coordinates(), "Latitude 91 should be invalid");
}

#[test]
fn longitude_out_of_range_rejected() {
    let loc = Location::new(0.0, 181.0, 0.0, 5.0);
    assert!(!loc.is_valid_coordinates(), "Longitude 181 should be invalid");
}

#[test]
fn negative_longitude_valid() {
    let loc = Location::new(0.0, -179.9, 0.0, 5.0);
    assert!(loc.is_valid_coordinates());
}

#[test]
fn accuracy_threshold_pass() {
    let loc = Location::new(37.0, -122.0, 0.0, 50.0);
    assert!(loc.is_accurate_enough(100.0));
}

#[test]
fn accuracy_threshold_fail() {
    let loc = Location::new(37.0, -122.0, 0.0, 150.0);
    assert!(!loc.is_accurate_enough(100.0));
}

#[test]
fn accuracy_exactly_on_threshold() {
    let loc = Location::new(37.0, -122.0, 0.0, 100.0);
    assert!(loc.is_accurate_enough(100.0), "Exactly on threshold should pass");
}
