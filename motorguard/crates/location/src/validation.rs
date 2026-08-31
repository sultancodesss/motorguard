use motorguard_core::{models::Location, AppError, Result};

/// Maximum acceptable GPS accuracy radius in metres for ride tracking.
pub const DEFAULT_ACCURACY_THRESHOLD_M: f64 = 100.0;

/// Maximum plausible motorcycle speed in mph (used to reject sensor noise).
pub const MAX_PLAUSIBLE_SPEED_MPH: f64 = 200.0;

/// Validate a location before it is stored or broadcast.
pub fn validate_location(loc: &Location) -> Result<()> {
    if !loc.is_valid_coordinates() {
        return Err(AppError::InvalidCoordinates);
    }

    if loc.speed < 0.0 || loc.speed > MAX_PLAUSIBLE_SPEED_MPH {
        return Err(AppError::Validation(format!(
            "Speed {} mph is outside plausible range",
            loc.speed
        )));
    }

    if loc.accuracy < 0.0 {
        return Err(AppError::Validation("Accuracy cannot be negative".to_string()));
    }

    Ok(())
}

/// Check whether accuracy is good enough for ride tracking.
/// Poor accuracy points are accepted but flagged for the safety score.
pub fn is_tracking_quality(loc: &Location) -> bool {
    loc.is_accurate_enough(DEFAULT_ACCURACY_THRESHOLD_M)
}
