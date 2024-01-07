use hugsqlx::{HugSqlx, params};
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, Pool};
use uuid::Uuid;

use crate::errors::InternalError;

use super::user::User;

#[derive(HugSqlx)]
#[queries = "resources/db/queries/bookmarks.sql"]
struct Bookmarks {}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Bookmark {
    #[sqlx(rename = "link_id")]
    id: Uuid,
    category_id: Uuid,
    name: String,
    url: String,
    icon: String,
    visible: bool,
    position: u16
}

pub async fn fetch_bookmarks(pool: &Pool<Sqlite>, user: &User) -> anyhow::Result<Vec<Bookmark>> {
    Ok(Bookmarks::fetch_bookmarks_for_user_id::<_, Bookmark>(pool, params!(user.id)).await.map_err(|e| {
        tracing::error!("Could load user's applications: ${e}");
        InternalError::UserLinksFetchError
    })?)
}
