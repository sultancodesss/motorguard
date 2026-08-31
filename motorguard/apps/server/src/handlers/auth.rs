use axum::{extract::State, http::StatusCode, Json};
use motorguard_api::{
    requests::{RefreshTokenBody, RequestOtpBody, VerifyOtpBody},
    responses::{ApiResponse, AuthResponse, RefreshResponse, RequestOtpResponse, UserResponse},
};

use crate::{errors::ApiResult, state::AppState};

/// POST /api/auth/request-otp
pub async fn request_otp(
    State(state): State<AppState>,
    Json(body): Json<RequestOtpBody>,
) -> ApiResult<(StatusCode, Json<ApiResponse<RequestOtpResponse>>)> {
    // Generate OTP — the plaintext is only returned in dev mode logs
    state.auth.request_otp(&body.phone).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::ok(RequestOtpResponse {
            message: "OTP sent to your phone".to_string(),
            expires_in_seconds: 600,
        })),
    ))
}

/// POST /api/auth/verify-otp
pub async fn verify_otp(
    State(state): State<AppState>,
    Json(body): Json<VerifyOtpBody>,
) -> ApiResult<(StatusCode, Json<ApiResponse<AuthResponse>>)> {
    let tokens = state.auth.verify_otp(&body.phone, &body.code).await?;

    let resp = AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: tokens.expires_in,
        user: UserResponse {
            id: tokens.user.id.to_string(),
            phone: tokens.user.phone,
            name: tokens.user.name,
            avatar_url: tokens.user.avatar_url,
            created_at: tokens.user.created_at,
            stats: None,
        },
        is_new_user: tokens.is_new_user,
    };

    Ok((StatusCode::OK, Json(ApiResponse::ok(resp))))
}

/// POST /api/auth/refresh
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(body): Json<RefreshTokenBody>,
) -> ApiResult<Json<ApiResponse<RefreshResponse>>> {
    let (access_token, expires_in) =
        state.auth.refresh_access_token(&body.refresh_token).await?;

    Ok(Json(ApiResponse::ok(RefreshResponse {
        access_token,
        expires_in,
    })))
}

/// POST /api/auth/logout
pub async fn logout() -> StatusCode {
    // Client-side: discard tokens. Server-side session cleanup can be
    // added here when session IDs are tracked in cookies.
    StatusCode::NO_CONTENT
}
