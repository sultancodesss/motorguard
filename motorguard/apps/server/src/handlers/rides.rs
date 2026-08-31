use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use motorguard_api::{
    requests::{AddRidePointsBody, CreateRideBody, ListRidesQuery},
    responses::{AddPointsResponse, ApiResponse, RideListResponse, RideResponse},
};
use motorguard_core::models::Location;
use uuid::Uuid;

use crate::{errors::ApiResult, middleware::AuthUser, state::AppState};

fn ride_to_response(r: motorguard_core::models::Ride) -> RideResponse {
    RideResponse {
        id: r.id.to_string(),
        name: r.name,
        status: r.status.to_string(),
        started_at: r.started_at,
        ended_at: r.ended_at,
        distance_miles: r.distance_miles,
        duration_seconds: r.duration_seconds,
        duration_display: r.duration_display(),
        average_speed_mph: r.average_speed_mph,
        max_speed_mph: r.max_speed_mph,
        safety_score: r.safety_score,
        route_summary: r.route_summary,
        created_at: r.created_at,
    }
}

/// POST /api/rides
pub async fn create_ride(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateRideBody>,
) -> ApiResult<(StatusCode, Json<ApiResponse<RideResponse>>)> {
    let ride = state.rides.create_ride(auth.user_id, body.name).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::ok(ride_to_response(ride)))))
}

/// GET /api/rides
pub async fn list_rides(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Query(q): Query<ListRidesQuery>,
) -> ApiResult<Json<ApiResponse<RideListResponse>>> {
    let rides = state.rides.list_rides(auth.user_id, q.page, q.per_page).await?;
    let count = rides.len();
    Ok(Json(ApiResponse::ok(RideListResponse {
        rides: rides.into_iter().map(ride_to_response).collect(),
        total: count,
        page: q.page,
        per_page: q.per_page,
    })))
}

/// GET /api/rides/:id
pub async fn get_ride(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(ride_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<RideResponse>>> {
    use motorguard_core::types::RideId;
    let ride = state
        .rides
        .get_ride(RideId::from_uuid(ride_id), auth.user_id)
        .await?;
    Ok(Json(ApiResponse::ok(ride_to_response(ride))))
}

/// POST /api/rides/:id/start
pub async fn start_ride(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(ride_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<RideResponse>>> {
    use motorguard_core::types::RideId;
    let ride = state
        .rides
        .start_ride(RideId::from_uuid(ride_id), auth.user_id)
        .await?;
    Ok(Json(ApiResponse::ok(ride_to_response(ride))))
}

/// POST /api/rides/:id/pause
pub async fn pause_ride(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(ride_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<RideResponse>>> {
    use motorguard_core::types::RideId;
    let ride = state
        .rides
        .pause_ride(RideId::from_uuid(ride_id), auth.user_id)
        .await?;
    Ok(Json(ApiResponse::ok(ride_to_response(ride))))
}

/// POST /api/rides/:id/resume
pub async fn resume_ride(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(ride_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<RideResponse>>> {
    use motorguard_core::types::RideId;
    let ride = state
        .rides
        .resume_ride(RideId::from_uuid(ride_id), auth.user_id)
        .await?;
    Ok(Json(ApiResponse::ok(ride_to_response(ride))))
}

/// POST /api/rides/:id/finish
pub async fn finish_ride(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(ride_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<RideResponse>>> {
    use motorguard_core::types::RideId;
    let ride = state
        .rides
        .finish_ride(RideId::from_uuid(ride_id), auth.user_id)
        .await?;
    Ok(Json(ApiResponse::ok(ride_to_response(ride))))
}

/// POST /api/rides/:id/points
pub async fn add_points(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(ride_id): Path<Uuid>,
    Json(body): Json<AddRidePointsBody>,
) -> ApiResult<Json<ApiResponse<AddPointsResponse>>> {
    use motorguard_core::types::RideId;

    let locations: Vec<Location> = body
        .points
        .into_iter()
        .map(|p| Location {
            latitude: p.latitude,
            longitude: p.longitude,
            altitude: p.altitude,
            speed: p.speed,
            heading: None,
            accuracy: p.accuracy,
            timestamp: p.timestamp,
        })
        .collect();

    let count = state
        .rides
        .add_points(RideId::from_uuid(ride_id), auth.user_id, locations)
        .await?;

    Ok(Json(ApiResponse::ok(AddPointsResponse {
        points_added: count,
    })))
}
