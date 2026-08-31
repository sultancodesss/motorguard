use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use motorguard_api::{
    requests::{CreateContactBody, ResolveSosBody, TriggerSosBody},
    responses::{
        ApiResponse, ContactListResponse, ContactResponse, ResolveSosResponse, SosResponse,
    },
};
use motorguard_core::types::SosTrigger;
use uuid::Uuid;

use crate::{errors::ApiResult, middleware::AuthUser, state::AppState};

/// POST /api/sos
pub async fn trigger_sos(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<TriggerSosBody>,
) -> ApiResult<(StatusCode, Json<ApiResponse<SosResponse>>)> {
    let trigger = match body.trigger.as_str() {
        "crash_detection" => SosTrigger::CrashDetection,
        _ => SosTrigger::Manual,
    };

    let result = state
        .safety
        .dispatch_sos(
            auth.user_id,
            body.latitude,
            body.longitude,
            body.accuracy,
            trigger,
        )
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::ok(SosResponse {
            id: result.event.id.to_string(),
            status: format!("{:?}", result.event.status).to_lowercase(),
            latitude: result.event.latitude,
            longitude: result.event.longitude,
            contacts_notified: result.event.contacts_notified,
            created_at: result.event.created_at,
            message: format!(
                "SOS dispatched. {} contact(s) notified.",
                result.contacts_notified
            ),
        })),
    ))
}

/// POST /api/sos/:id/resolve
pub async fn resolve_sos(
    State(state): State<AppState>,
    Path(sos_id): Path<Uuid>,
    Json(body): Json<ResolveSosBody>,
) -> ApiResult<Json<ApiResponse<ResolveSosResponse>>> {
    use motorguard_core::types::SosEventId;
    state
        .safety
        .resolve_sos(SosEventId::from_uuid(sos_id), &body.reason)
        .await?;
    Ok(Json(ApiResponse::ok(ResolveSosResponse {
        message: "SOS event resolved".to_string(),
    })))
}

/// GET /api/emergency-contacts
pub async fn list_contacts(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> ApiResult<Json<ApiResponse<ContactListResponse>>> {
    let contacts = state.safety.list_contacts(auth.user_id).await?;
    Ok(Json(ApiResponse::ok(ContactListResponse {
        contacts: contacts.into_iter().map(contact_to_response).collect(),
    })))
}

/// POST /api/emergency-contacts
pub async fn create_contact(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateContactBody>,
) -> ApiResult<(StatusCode, Json<ApiResponse<ContactResponse>>)> {
    let contact = state
        .safety
        .add_contact(auth.user_id, body.name, body.phone, body.relationship)
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::ok(contact_to_response(contact))),
    ))
}

/// DELETE /api/emergency-contacts/:id
pub async fn delete_contact(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(contact_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    use motorguard_core::types::EmergencyContactId;
    state
        .safety
        .remove_contact(EmergencyContactId::from_uuid(contact_id), auth.user_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

fn contact_to_response(c: motorguard_core::models::EmergencyContact) -> ContactResponse {
    ContactResponse {
        id: c.id.to_string(),
        name: c.name,
        phone: c.phone,
        relationship: c.relationship,
        created_at: c.created_at,
    }
}
