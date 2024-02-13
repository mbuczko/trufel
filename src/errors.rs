use alcoholic_jwt::ValidationError;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use thiserror::Error;

// Make our own error that wraps `anyhow::Error`.
pub struct ServiceError(anyhow::Error);

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("User update failed")]
    UserUpdate,

    #[error("User's apps couldn't be fetched")]
    AppsFetch,

    #[error("User's links couldn't be fetched")]
    LinksFetch,

    #[error("User's categories couldn't be fetched")]
    CategoriesFetch,

    #[error("New category couldn't be created")]
    CategoriesCreate,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Invalid claims")]
    InvalidClaims,

    #[error("JWT validation error")]
    JWTValidationError(ValidationError),

    #[error("JWKS fetching error")]
    JWKSFetchError,

    #[error("JWKS deserialization error")]
    JWKSDeserializeError,
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response<Body> {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

/// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
/// `Result<_, ServiceError>`. That way we don't need to do that manually.
impl<E> From<E> for ServiceError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
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
