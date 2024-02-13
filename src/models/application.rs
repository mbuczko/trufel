use hugsqlx::{params, HugSqlx};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::errors::InternalError;

use super::user::User;

#[derive(HugSqlx)]
#[queries = "resources/db/queries/applications.sql"]
struct Applications {}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Application {
    #[sqlx(rename = "application_id")]
    id: Uuid,
    name: String,
    description: Option<String>,
    url: String,
    icon: String,
    visible: bool,
    position: u16,
}

pub async fn fetch_applications(
    pool: &Pool<Sqlite>,
    user: &User,
) -> anyhow::Result<Vec<Application>> {
    Ok(
        Applications::fetch_applications_for_user_id::<_, Application>(pool, params!(user.id))
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "Could load user's applications");
                InternalError::AppsFetch
            })?,
    )
}
