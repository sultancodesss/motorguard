use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ── Envelope ─────────────────────────────────────────────────────────────────

/// Standard success envelope — all 2xx responses use this.
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { data }
    }
}

/// Standard error envelope — all 4xx/5xx responses use this.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: ErrorDetail {
                code: code.into(),
                message: message.into(),
            },
        }
    }
}

// ── Auth ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct RequestOtpResponse {
    pub message: String,
    pub expires_in_seconds: i64,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
    pub is_new_user: bool,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub expires_in: i64,
}

// ── Users ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub phone: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub stats: Option<UserStatsResponse>,
}

#[derive(Debug, Serialize)]
pub struct UserStatsResponse {
    pub total_rides: i64,
    pub total_miles: f64,
    pub safety_score: f64,
}

#[derive(Debug, Serialize)]
pub struct MotorcycleResponse {
    pub id: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub plate: Option<String>,
    pub color: Option<String>,
    pub display_name: String,
}

// ── Rides ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct RideResponse {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub distance_miles: f64,
    pub duration_seconds: i64,
    pub duration_display: String,
    pub average_speed_mph: f64,
    pub max_speed_mph: f64,
    pub safety_score: Option<f64>,
    pub route_summary: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RideListResponse {
    pub rides: Vec<RideResponse>,
    pub total: usize,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize)]
pub struct AddPointsResponse {
    pub points_added: usize,
}

// ── Groups ───────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub invite_code: String,
    pub member_count: i64,
    pub is_member: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct GroupListResponse {
    pub groups: Vec<GroupResponse>,
}

#[derive(Debug, Serialize)]
pub struct JoinGroupResponse {
    pub message: String,
}

// ── SOS ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct SosResponse {
    pub id: String,
    pub status: String,
    pub latitude: f64,
    pub longitude: f64,
    pub contacts_notified: i32,
    pub created_at: DateTime<Utc>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ResolveSosResponse {
    pub message: String,
}

// ── Emergency Contacts ───────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub relationship: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ContactListResponse {
    pub contacts: Vec<ContactResponse>,
}
