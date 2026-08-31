use axum::{extract::State, Extension, Json};
use motorguard_api::{
    requests::UpdateProfileBody,
    responses::{ApiResponse, UserResponse, UserStatsResponse},
};
use motorguard_database::UserRepository;

use crate::{errors::ApiResult, middleware::AuthUser, state::AppState};

/// GET /api/users/me
pub async fn get_me(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<ApiResponse<UserResponse>>> {
    let users = UserRepository::new(&state.db);
    let user = users.find_by_id(auth.user_id).await?;
    let stats = users.get_stats(auth.user_id).await?;

    Ok(Json(ApiResponse::ok(UserResponse {
        id: user.id.to_string(),
        phone: user.phone,
        name: user.name,
        avatar_url: user.avatar_url,
        created_at: user.created_at,
        stats: Some(UserStatsResponse {
            total_rides: stats.total_rides,
            total_miles: stats.total_miles,
            safety_score: stats.safety_score,
        }),
    })))
}

/// PUT /api/users/me
pub async fn update_me(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<UpdateProfileBody>,
) -> ApiResult<Json<ApiResponse<UserResponse>>> {
    let users = UserRepository::new(&state.db);
    let updated = users
        .update_profile(auth.user_id, body.name, body.avatar_url)
        .await?;

    Ok(Json(ApiResponse::ok(UserResponse {
        id: updated.id.to_string(),
        phone: updated.phone,
        name: updated.name,
        avatar_url: updated.avatar_url,
        created_at: updated.created_at,
        stats: None,
    })))
}
