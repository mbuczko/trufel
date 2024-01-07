use axum::{extract::Form, extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{errors::AuthError, models::user::{User, self}};

pub async fn user_update(
    State(pool): State<SqlitePool>,
    Form(user): Form<User>,
) -> Result<Json<User>, AuthError> {
    tracing::info!("Updating user's profile... {:?}", user);

    let user = user::store(&pool, user).await.map_err(|e| {
        tracing::error!("Could not store user's profile: {}", e);
        AuthError::JWKSFetchError
    })?;

    Ok(Json(user))
}

pub async fn user_identity(user: User) -> Result<Json<User>, StatusCode> {
    tracing::info!("Fetching user's profile");
    Ok(Json(user))
}
