use thiserror::Error;

/// The canonical application error type.
///
/// Every crate maps its internal errors into `AppError` before returning
/// across a crate boundary so that handlers always deal with a single
/// typed error.
#[derive(Debug, Error)]
pub enum AppError {
    // ── Authentication ──────────────────────────────────────────────────────
    #[error("Authentication required")]
    Unauthorized,

    #[error("Access denied")]
    Forbidden,

    #[error("OTP code is invalid")]
    OtpInvalid,

    #[error("OTP code has expired")]
    OtpExpired,

    #[error("Session has expired, please log in again")]
    SessionExpired,

    #[error("Token is malformed")]
    TokenMalformed,

    // ── Validation ──────────────────────────────────────────────────────────
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Phone number format is invalid (must be E.164)")]
    InvalidPhone,

    #[error("Coordinates are out of valid range")]
    InvalidCoordinates,

    // ── Not Found ───────────────────────────────────────────────────────────
    #[error("{0} not found")]
    NotFound(String),

    // ── Conflict ────────────────────────────────────────────────────────────
    #[error("Already a member of this group")]
    AlreadyMember,

    #[error("Phone number already registered")]
    PhoneAlreadyExists,

    // ── Ride ────────────────────────────────────────────────────────────────
    #[error("Ride is not in active state")]
    RideNotActive,

    #[error("Ride is already active")]
    RideAlreadyActive,

    #[error("Cannot perform this action on a completed ride")]
    RideAlreadyCompleted,

    // ── Location ────────────────────────────────────────────────────────────
    #[error("Location permission denied")]
    LocationPermissionDenied,

    #[error("GPS accuracy too poor: {accuracy}m (threshold: {threshold}m)")]
    LocationAccuracyPoor { accuracy: f64, threshold: f64 },

    // ── SOS ─────────────────────────────────────────────────────────────────
    #[error("SOS event not found")]
    SosNotFound,

    #[error("SOS event is already resolved")]
    SosAlreadyResolved,

    // ── Rate Limiting ───────────────────────────────────────────────────────
    #[error("Too many requests, please try again later")]
    RateLimited,

    // ── Database ────────────────────────────────────────────────────────────
    #[error("Database error: {0}")]
    Database(String),

    #[error("Migration failed: {0}")]
    Migration(String),

    // ── Network ─────────────────────────────────────────────────────────────
    #[error("Network request failed: {0}")]
    Network(String),

    #[error("External service unavailable: {0}")]
    ServiceUnavailable(String),

    // ── Internal ────────────────────────────────────────────────────────────
    #[error("Internal server error")]
    Internal(#[source] anyhow::Error),

    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
}

impl AppError {
    /// HTTP status code that should be returned for this error.
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::Unauthorized
            | AppError::SessionExpired
            | AppError::TokenMalformed => 401,

            AppError::Forbidden => 403,

            AppError::NotFound(_) | AppError::SosNotFound => 404,

            AppError::AlreadyMember | AppError::PhoneAlreadyExists | AppError::RideAlreadyActive => 409,

            AppError::OtpInvalid
            | AppError::OtpExpired
            | AppError::Validation(_)
            | AppError::InvalidPhone
            | AppError::InvalidCoordinates
            | AppError::RideNotActive
            | AppError::RideAlreadyCompleted
            | AppError::SosAlreadyResolved
            | AppError::LocationPermissionDenied
            | AppError::LocationAccuracyPoor { .. } => 422,

            AppError::RateLimited => 429,

            AppError::Database(_)
            | AppError::Migration(_)
            | AppError::Network(_)
            | AppError::ServiceUnavailable(_)
            | AppError::Internal(_)
            | AppError::NotImplemented(_) => 500,
        }
    }

    /// Machine-readable error code for JSON responses.
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Unauthorized => "UNAUTHORIZED",
            AppError::Forbidden => "FORBIDDEN",
            AppError::OtpInvalid => "OTP_INVALID",
            AppError::OtpExpired => "OTP_EXPIRED",
            AppError::SessionExpired => "SESSION_EXPIRED",
            AppError::TokenMalformed => "TOKEN_MALFORMED",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::InvalidPhone => "INVALID_PHONE",
            AppError::InvalidCoordinates => "INVALID_COORDINATES",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::AlreadyMember => "ALREADY_MEMBER",
            AppError::PhoneAlreadyExists => "PHONE_EXISTS",
            AppError::RideNotActive => "RIDE_NOT_ACTIVE",
            AppError::RideAlreadyActive => "RIDE_ALREADY_ACTIVE",
            AppError::RideAlreadyCompleted => "RIDE_COMPLETED",
            AppError::LocationPermissionDenied => "LOCATION_PERMISSION_DENIED",
            AppError::LocationAccuracyPoor { .. } => "LOCATION_ACCURACY_POOR",
            AppError::SosNotFound => "SOS_NOT_FOUND",
            AppError::SosAlreadyResolved => "SOS_ALREADY_RESOLVED",
            AppError::RateLimited => "RATE_LIMITED",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Migration(_) => "MIGRATION_ERROR",
            AppError::Network(_) => "NETWORK_ERROR",
            AppError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::NotImplemented(_) => "NOT_IMPLEMENTED",
        }
    }
}

/// Convenience Result alias used throughout the application.
pub type Result<T> = std::result::Result<T, AppError>;

// ── Conversion helpers ───────────────────────────────────────────────────────

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound("record".to_string()),
            _ => AppError::Database(e.to_string()),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Internal(e)
    }
}
