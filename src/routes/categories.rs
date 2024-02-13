use axum::{extract::State, Json};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
    errors::ServiceError,
    models::{
        category::{create_category, fetch_categories, Category},
        user::User,
    },
};

#[derive(Deserialize)]
pub struct NewCategoryRequest {
    name: String
}

pub async fn categories(
    State(pool): State<SqlitePool>,
    user: User,
) -> Result<Json<Vec<Category>>, ServiceError> {
    Ok(Json(fetch_categories(&pool, &user).await?))
}

pub async fn add_category(
    State(pool): State<SqlitePool>,
    user: User,
    Json(category): Json<NewCategoryRequest>
) -> Result<Json<Category>, ServiceError> {
    let category = create_category(&pool, &user, category.name).await?;
    Ok(Json(category))
}
