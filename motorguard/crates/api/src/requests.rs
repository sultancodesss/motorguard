use serde::{Deserialize, Serialize};
use validator::Validate;

// ── Auth ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct RequestOtpBody {
    #[validate(length(min = 8, max = 16, message = "Phone must be E.164 format"))]
    pub phone: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyOtpBody {
    #[validate(length(min = 8, max = 16))]
    pub phone: String,
    #[validate(length(equal = 6, message = "Code must be 6 digits"))]
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenBody {
    pub refresh_token: String,
}

// ── Users ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileBody {
    #[validate(length(min = 1, max = 60, message = "Name must be 1–60 characters"))]
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMotorcycleBody {
    #[validate(length(min = 1, max = 50))]
    pub make: String,
    #[validate(length(min = 1, max = 50))]
    pub model: String,
    #[validate(range(min = 1900, max = 2100))]
    pub year: i32,
    pub plate: Option<String>,
    pub color: Option<String>,
}

// ── Rides ────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateRideBody {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub speed: f64,
    pub accuracy: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AddRidePointsBody {
    pub points: Vec<LocationPoint>,
}

#[derive(Debug, Deserialize)]
pub struct ListRidesQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 { 1 }
fn default_per_page() -> i64 { 20 }

// ── Groups ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGroupBody {
    #[validate(length(min = 1, max = 80, message = "Name must be 1–80 characters"))]
    pub name: String,
    #[validate(length(max = 300))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JoinGroupBody {
    pub invite_code: Option<String>,
}

// ── Locations ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SubmitLocationBody {
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub accuracy: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ── SOS ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TriggerSosBody {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    #[serde(default = "default_trigger")]
    pub trigger: String,
}

fn default_trigger() -> String { "manual".to_string() }

#[derive(Debug, Deserialize)]
pub struct ResolveSosBody {
    #[serde(default = "default_resolve_reason")]
    pub reason: String,
}

fn default_resolve_reason() -> String { "false_alarm".to_string() }

// ── Emergency Contacts ───────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct CreateContactBody {
    #[validate(length(min = 1, max = 80))]
    pub name: String,
    #[validate(length(min = 8, max = 16))]
    pub phone: String,
    pub relationship: Option<String>,
}
