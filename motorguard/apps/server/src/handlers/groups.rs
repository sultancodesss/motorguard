use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use motorguard_api::{
    requests::{CreateGroupBody, JoinGroupBody},
    responses::{ApiResponse, GroupListResponse, GroupResponse, JoinGroupResponse},
};
use uuid::Uuid;

use crate::{errors::ApiResult, middleware::AuthUser, state::AppState};

async fn group_response(
    state: &AppState,
    group: motorguard_core::models::Group,
    user_id: motorguard_core::types::UserId,
) -> ApiResult<GroupResponse> {
    let member_count = state.groups.get_member_count(group.id).await?;
    let is_member = state
        .groups
        .assert_member(group.id, user_id)
        .await
        .is_ok();

    Ok(GroupResponse {
        id: group.id.to_string(),
        name: group.name,
        description: group.description,
        invite_code: group.invite_code,
        member_count,
        is_member,
        created_at: group.created_at,
    })
}

/// GET /api/groups
pub async fn list_groups(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<ApiResponse<GroupListResponse>>> {
    let groups = state.groups.list_user_groups(auth.user_id).await?;

    let mut responses = Vec::with_capacity(groups.len());
    for g in groups {
        responses.push(group_response(&state, g, auth.user_id).await?);
    }

    Ok(Json(ApiResponse::ok(GroupListResponse { groups: responses })))
}

/// POST /api/groups
pub async fn create_group(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateGroupBody>,
) -> ApiResult<(StatusCode, Json<ApiResponse<GroupResponse>>)> {
    let group = state
        .groups
        .create_group(auth.user_id, body.name, body.description)
        .await?;
    let resp = group_response(&state, group, auth.user_id).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(resp))))
}

/// GET /api/groups/:id
pub async fn get_group(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(group_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<GroupResponse>>> {
    use motorguard_core::types::GroupId;
    let group = state.groups.get_group(GroupId::from_uuid(group_id)).await?;
    let resp = group_response(&state, group, auth.user_id).await?;
    Ok(Json(ApiResponse::ok(resp)))
}

/// POST /api/groups/:id/join
pub async fn join_group(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(group_id): Path<Uuid>,
    Json(body): Json<JoinGroupBody>,
) -> ApiResult<Json<ApiResponse<JoinGroupResponse>>> {
    use motorguard_core::types::GroupId;
    state
        .groups
        .join_group(GroupId::from_uuid(group_id), auth.user_id, body.invite_code)
        .await?;
    Ok(Json(ApiResponse::ok(JoinGroupResponse {
        message: "Joined group successfully".to_string(),
    })))
}

/// POST /api/groups/:id/leave
pub async fn leave_group(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(group_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    use motorguard_core::types::GroupId;
    state
        .groups
        .leave_group(GroupId::from_uuid(group_id), auth.user_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
