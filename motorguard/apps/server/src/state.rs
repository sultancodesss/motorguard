use motorguard_auth::{AuthService, JwtService};
use motorguard_groups::{GroupChannelRegistry, GroupService};
use motorguard_location::LocationService;
use motorguard_notifications::NotificationService;
use motorguard_rides::RideService;
use motorguard_safety::SafetyService;
use std::sync::Arc;

/// Shared application state injected into every Axum handler.
#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<AuthService>,
    pub jwt: Arc<JwtService>,
    pub rides: Arc<RideService>,
    pub groups: Arc<GroupService>,
    pub safety: Arc<SafetyService>,
    pub location: Arc<LocationService>,
    pub channels: Arc<GroupChannelRegistry>,
    pub db: motorguard_database::DbPool,
}
