use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::{
    errors::ServiceError,
    models::{
        application::{Application, fetch_applications},
        bookmark::{Bookmark, fetch_bookmarks},
        category::{Category, fetch_categories},
        user::User,
    },
};

#[derive(Serialize, Debug)]
pub struct Components {
    applications: Vec<Application>,
    links: Vec<Bookmark>,
    categories: Vec<Category>,
}
pub async fn fetch_components(
    State(pool): State<SqlitePool>,
    user: User,
) -> Result<Json<Components>, ServiceError> {
    tracing::info!("Fetching user's bookmarks");

    let applications = fetch_applications(&pool, &user).await?;
    let links = fetch_bookmarks(&pool, &user).await?;
    let categories = fetch_categories(&pool, &user).await?;

    Ok(Json(Components {
        applications,
        links,
        categories,
    }))
}
