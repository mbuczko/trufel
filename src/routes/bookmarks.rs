use axum::{Json, extract::State};
use sqlx::SqlitePool;

use crate::{models::{user::User, application::{self, Application}}, errors::ServiceError};

pub async fn fetch_bookmarks(State(pool): State<SqlitePool>, user: User) -> Result<Json<Vec<Application>>, ServiceError> {
    tracing::info!("Fetching user's bookmarks");

    let apps = application::fetch_applications(&pool, user).await?;
    Ok(Json(apps))
}
