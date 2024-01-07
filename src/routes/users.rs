use axum::{extract::Form, extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

use crate::{
    errors::ServiceError,
    models::user::{self, User},
};

pub async fn user_update(
    State(pool): State<SqlitePool>,
    Form(user): Form<User>,
) -> Result<Json<User>, ServiceError> {
    tracing::info!(user = ?user, "Updating user's profile");

    let user = user::store(&pool, user).await?;
    Ok(Json(user))
}

pub async fn user_identity(user: User) -> Result<Json<User>, StatusCode> {
    tracing::info!("Fetching user's profile");
    Ok(Json(user))
}
