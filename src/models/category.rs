use hugsqlx::{params, HugSqlx};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::errors::InternalError;

use super::user::User;

#[derive(HugSqlx)]
#[queries = "resources/db/queries/categories.sql"]
struct Categories {}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Category {
    #[sqlx(rename = "category_id")]
    id: Uuid,
    name: String,
}

pub async fn fetch_categories(pool: &Pool<Sqlite>, user: &User) -> anyhow::Result<Vec<Category>> {
    Ok(
        Categories::fetch_categories_for_user_id::<_, Category>(pool, params!(user.id))
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Could load user's applications");
                InternalError::UserCategoriesFetchError
            })?,
    )
}
