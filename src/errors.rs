use alcoholic_jwt::ValidationError;
use axum::{response::IntoResponse, http::{Response, StatusCode}, body::BoxBody, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("JWT validation error")]
    JWTValidationError(ValidationError),

    #[error("JWKS fetching error")]
    JWKSFetchError,

    #[error("JWKS deserialization error")]
    JWKSDeserializeError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<BoxBody> {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::JWKSDeserializeError => (StatusCode::SERVICE_UNAVAILABLE, "Cannot deserivalize JWKS"),
            AuthError::JWKSFetchError => (StatusCode::SERVICE_UNAVAILABLE, "Cannot fetch JWKS"),
            AuthError::JWTValidationError(_) => (StatusCode::SERVICE_UNAVAILABLE, "Cannot validate token")
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
