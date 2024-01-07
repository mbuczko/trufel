use alcoholic_jwt::ValidationError;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("User not found")]
    UserNotFound,
}

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

    #[error("Invalid claims")]
    InvalidClaims,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::JWKSFetchError => (StatusCode::SERVICE_UNAVAILABLE, "Cannot fetch JWKS"),
            AuthError::JWKSDeserializeError => {
                (StatusCode::SERVICE_UNAVAILABLE, "Cannot deserivalize JWKS")
            }
            AuthError::JWTValidationError(_) => {
                (StatusCode::SERVICE_UNAVAILABLE, "Cannot validate token")
            }
            AuthError::InvalidClaims => (StatusCode::UNAUTHORIZED, "No valid claims found"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
