use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use motorguard_api::responses::ErrorResponse;
use motorguard_core::AppError;

/// Newtype so we can implement `IntoResponse` for `AppError`.
pub struct ServerError(pub AppError);

impl From<AppError> for ServerError {
    fn from(e: AppError) -> Self {
        Self(e)
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.0.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = ErrorResponse::new(self.0.error_code(), self.0.to_string());

        (status, Json(body)).into_response()
    }
}

/// Convenience alias used in handler return types.
pub type ApiResult<T> = Result<T, ServerError>;
