use hugsqlx::{params, HugSqlx};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};
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
    position: u8
}

pub async fn fetch_categories(pool: &Pool<Sqlite>, user: &User) -> anyhow::Result<Vec<Category>> {
    Ok(
        Categories::fetch_categories_for_user_id::<_, Category>(pool, params!(user.id))
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Couldn't load user's categories");
                InternalError::CategoriesFetch
            })?
    )
}

pub async fn create_category(pool: &Pool<Sqlite>, user: &User, category_name: String) -> anyhow::Result<Category> {
    Ok(Categories::create_new_category(pool, params!(Uuid::new_v4(), user.id, &category_name))
       .await
       .map(|row| {
           let category_id = row.get(0);
           let position = row.get(1);

           Category {
               id: category_id,
               name: category_name,
               position
           }
       }).map_err(|e| {
           tracing::error!(error = ?e, "Couldn't create new category");
           InternalError::CategoriesCreate
       })?
    )
}
