use std::f64::consts::PI;

const EARTH_RADIUS_MILES: f64 = 3_958.8;
const EARTH_RADIUS_KM:    f64 = 6_371.0;

/// Haversine formula — great-circle distance in miles between two WGS84 points.
pub fn haversine_miles(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    haversine(lat1, lon1, lat2, lon2, EARTH_RADIUS_MILES)
}

/// Haversine formula — great-circle distance in kilometres.
pub fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    haversine(lat1, lon1, lat2, lon2, EARTH_RADIUS_KM)
}

fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64, radius: f64) -> f64 {
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let lat1_r = lat1.to_radians();
    let lat2_r = lat2.to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1_r.cos() * lat2_r.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    radius * c
}

/// Calculate speed in mph given distance in miles and elapsed seconds.
pub fn speed_mph(distance_miles: f64, elapsed_seconds: f64) -> f64 {
    if elapsed_seconds <= 0.0 {
        return 0.0;
    }
    let hours = elapsed_seconds / 3600.0;
    distance_miles / hours
}

/// Convert mph to km/h.
pub fn mph_to_kmh(mph: f64) -> f64 {
    mph * 1.60934
}

/// Convert km/h to mph.
pub fn kmh_to_mph(kmh: f64) -> f64 {
    kmh / 1.60934
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haversine_same_point_is_zero() {
        let d = haversine_miles(37.7749, -122.4194, 37.7749, -122.4194);
        assert!(d < 0.0001, "Same point should give ~0, got {d}");
    }

    #[test]
    fn haversine_nyc_to_la() {
        let d = haversine_miles(40.7128, -74.0060, 34.0522, -118.2437);
        assert!((d - 2450.0).abs() < 60.0, "NYC→LA ~2450 mi, got {d}");
    }

    #[test]
    fn speed_60_mph() {
        let s = speed_mph(60.0, 3600.0);
        assert!((s - 60.0).abs() < 0.001);
    }

    #[test]
    fn speed_zero_elapsed() {
        assert_eq!(speed_mph(10.0, 0.0), 0.0);
    }

    #[test]
    fn mph_kmh_round_trip() {
        let original = 75.0_f64;
        let back = kmh_to_mph(mph_to_kmh(original));
        assert!((back - original).abs() < 0.001);
    }

    #[test]
    fn mph_to_kmh_60() {
        let k = mph_to_kmh(60.0);
        assert!((k - 96.56).abs() < 0.1, "60 mph ≈ 96.56 km/h, got {k}");
    }
}
