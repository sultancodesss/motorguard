use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::{auth, groups, rides, sos, users, ws},
    middleware::require_auth,
    state::AppState,
};

/// Build the full application router.
pub fn build_router(state: AppState, static_dir: &str) -> Router {
    // Protected routes — require a valid Bearer token
    let protected = Router::new()
        // Users
        .route("/api/users/me", get(users::get_me).put(users::update_me))
        // Rides
        .route("/api/rides", post(rides::create_ride).get(rides::list_rides))
        .route("/api/rides/:id", get(rides::get_ride))
        .route("/api/rides/:id/start", post(rides::start_ride))
        .route("/api/rides/:id/pause", post(rides::pause_ride))
        .route("/api/rides/:id/resume", post(rides::resume_ride))
        .route("/api/rides/:id/finish", post(rides::finish_ride))
        .route("/api/rides/:id/points", post(rides::add_points))
        // Groups
        .route("/api/groups", get(groups::list_groups).post(groups::create_group))
        .route("/api/groups/:id", get(groups::get_group))
        .route("/api/groups/:id/join", post(groups::join_group))
        .route("/api/groups/:id/leave", post(groups::leave_group))
        // SOS
        .route("/api/sos", post(sos::trigger_sos))
        .route("/api/sos/:id/resolve", post(sos::resolve_sos))
        // Emergency contacts
        .route(
            "/api/emergency-contacts",
            get(sos::list_contacts).post(sos::create_contact),
        )
        .route("/api/emergency-contacts/:id", delete(sos::delete_contact))
        // Apply auth middleware to all protected routes
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Public routes — no auth required
    let public = Router::new()
        .route("/api/auth/request-otp", post(auth::request_otp))
        .route("/api/auth/verify-otp", post(auth::verify_otp))
        .route("/api/auth/refresh", post(auth::refresh_token))
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/health", get(health_check));

    // WebSocket route (auth via query param, not header)
    let ws_route = Router::new().route("/ws/groups/:group_id", get(ws::group_ws_handler));

    // Static file serving — serves the mobile HTML/JS frontend
    let static_service = tower_http::services::ServeDir::new(static_dir)
        .append_index_html_on_directories(true);

    Router::new()
        .merge(public)
        .merge(protected)
        .merge(ws_route)
        .nest_service("/", static_service)
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}
