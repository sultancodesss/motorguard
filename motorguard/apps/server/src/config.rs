use std::env;

/// All runtime configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub jwt_refresh_expiry_days: i64,
    pub twilio_sid: Option<String>,
    pub twilio_token: Option<String>,
    pub twilio_from: Option<String>,
    pub enable_real_sos: bool,
    pub dev_mode: bool,
    pub allowed_origins: Vec<String>,
    pub otp_expiry_minutes: i64,
    pub static_dir: String,
}

impl Config {
    /// Load config from environment. Panics with a clear message on any
    /// missing required variable so misconfiguration is caught at startup.
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            host: env_or("HOST", "0.0.0.0"),
            port: env_or("PORT", "8080").parse().expect("PORT must be a number"),
            database_url: require_env("DATABASE_URL"),
            jwt_secret: require_env("JWT_SECRET"),
            jwt_expiry_hours: env_or("JWT_EXPIRY_HOURS", "24")
                .parse()
                .expect("JWT_EXPIRY_HOURS must be a number"),
            jwt_refresh_expiry_days: env_or("JWT_REFRESH_EXPIRY_DAYS", "30")
                .parse()
                .expect("JWT_REFRESH_EXPIRY_DAYS must be a number"),
            twilio_sid: env::var("TWILIO_ACCOUNT_SID").ok(),
            twilio_token: env::var("TWILIO_AUTH_TOKEN").ok(),
            twilio_from: env::var("TWILIO_FROM_NUMBER").ok(),
            enable_real_sos: env_or("ENABLE_REAL_SOS", "false")
                .to_lowercase()
                == "true",
            dev_mode: env_or("APP_ENV", "development").to_lowercase()
                != "production",
            allowed_origins: env_or("ALLOWED_ORIGINS", "http://localhost:8080")
                .split(',')
                .map(str::trim)
                .map(String::from)
                .collect(),
            otp_expiry_minutes: env_or("OTP_EXPIRY_MINUTES", "10")
                .parse()
                .expect("OTP_EXPIRY_MINUTES must be a number"),
            static_dir: env_or("STATIC_DIR", "apps/mobile"),
        }
    }
}

fn require_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Required env var '{key}' is not set"))
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
