use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use motorguard_core::types::UserId;

use crate::{errors::ServerError, state::AppState};

/// Extension type inserted by the auth middleware — handlers extract this.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: UserId,
}

/// Axum middleware that validates the Bearer token and inserts `AuthUser`.
pub async fn require_auth(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut req: Request,
    next: Next,
) -> Result<Response, ServerError> {
    let claims = state
        .jwt
        .verify_access_token(bearer.token())
        .map_err(ServerError::from)?;

    let user_id = claims.user_id().map_err(ServerError::from)?;

    req.extensions_mut().insert(AuthUser { user_id });
    Ok(next.run(req).await)
}
