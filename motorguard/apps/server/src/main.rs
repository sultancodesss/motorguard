mod config;
mod errors;
mod handlers;
mod middleware;
mod routes;
mod state;

use std::{net::SocketAddr, sync::Arc};

use motorguard_auth::{AuthService, JwtService, OtpService};
use motorguard_database::{create_pool, run_migrations};
use motorguard_groups::{GroupChannelRegistry, GroupService};
use motorguard_location::LocationService;
use motorguard_notifications::NotificationService;
use motorguard_rides::RideService;
use motorguard_safety::SafetyService;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{config::Config, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ── Logging ──────────────────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    // ── Config ───────────────────────────────────────────────────────────────
    let config = Config::from_env();
    info!("Starting MotorGuard server on {}:{}", config.host, config.port);
    info!(
        "Environment: {}",
        if config.dev_mode { "development" } else { "production" }
    );

    // ── Database ─────────────────────────────────────────────────────────────
    let pool = create_pool(&config.database_url).await?;
    run_migrations(&pool).await?;

    // ── Services ─────────────────────────────────────────────────────────────
    let jwt = Arc::new(JwtService::new(
        &config.jwt_secret,
        config.jwt_expiry_hours,
        config.jwt_refresh_expiry_days,
    ));

    let otp = OtpService::new(pool.clone(), config.otp_expiry_minutes, config.dev_mode);

    let auth = Arc::new(AuthService::new(
        pool.clone(),
        (*jwt).clone(),
        otp,
        config.jwt_refresh_expiry_days,
    ));

    let notifier = NotificationService::new(
        config.twilio_sid.clone(),
        config.twilio_token.clone(),
        config.twilio_from.clone(),
        config.dev_mode,
    );

    let safety = Arc::new(SafetyService::new(
        pool.clone(),
        notifier,
        config.enable_real_sos,
    ));

    let location = Arc::new(LocationService::new());

    let rides = Arc::new(RideService::new(pool.clone(), (*location).clone()));

    let groups = Arc::new(GroupService::new(pool.clone()));

    let channels = Arc::new(GroupChannelRegistry::new());

    // ── App State ────────────────────────────────────────────────────────────
    let state = AppState {
        auth,
        jwt,
        rides,
        groups,
        safety,
        location,
        channels,
        db: pool,
    };

    // ── CORS ─────────────────────────────────────────────────────────────────
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ── Router ───────────────────────────────────────────────────────────────
    let app = routes::build_router(state, &config.static_dir)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(cors);

    // ── Listen ───────────────────────────────────────────────────────────────
    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid bind address");

    info!("Server listening on http://{}", addr);
    info!("Open http://localhost:{} to see the app", config.port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
