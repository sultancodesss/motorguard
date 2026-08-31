use motorguard_core::{models::Location, AppError, Result};
use tracing::{debug, warn};

use crate::validation::{is_tracking_quality, validate_location};

/// Stateless service that processes raw location data from clients.
#[derive(Clone, Default)]
pub struct LocationService;

impl LocationService {
    pub fn new() -> Self {
        Self
    }

    /// Validate and sanitise a location update.
    /// Returns the location if valid, or an error if fundamentally invalid.
    pub fn process(&self, loc: Location) -> Result<Location> {
        validate_location(&loc)?;

        if !is_tracking_quality(&loc) {
            warn!(
                "Poor GPS accuracy: {}m at ({}, {})",
                loc.accuracy, loc.latitude, loc.longitude
            );
        }

        debug!(
            "Location processed: ({:.5}, {:.5}) speed={:.1}mph acc={}m",
            loc.latitude, loc.longitude, loc.speed, loc.accuracy
        );
        Ok(loc)
    }

    /// Filter a batch of locations, removing obviously invalid points.
    pub fn filter_batch(&self, locations: Vec<Location>) -> Vec<Location> {
        locations
            .into_iter()
            .filter(|loc| validate_location(loc).is_ok())
            .collect()
    }

    /// Check if two consecutive locations represent a plausible movement.
    /// Rejects teleports (> 1 mile in < 1 second).
    pub fn is_plausible_movement(&self, from: &Location, to: &Location) -> bool {
        let dt = (to.timestamp - from.timestamp).num_seconds();
        if dt <= 0 {
            return false;
        }
        let dist = crate::distance::haversine_miles(
            from.latitude,
            from.longitude,
            to.latitude,
            to.longitude,
        );
        // More than 1 mile in 1 second = impossible
        !(dist > 1.0 && dt <= 1)
    }
}
