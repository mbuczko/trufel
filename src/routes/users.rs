use anyhow::bail;
use axum::{extract::State, extract::Form, http::StatusCode, Json};
use hugsqlx::{params, HugSqlx};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::str::FromStr;
use uuid::Uuid;

use crate::errors::AuthError;
use crate::errors::DbError;
use crate::jwt::Claims;
use crate::models::user::User;

#[derive(HugSqlx)]
#[queries = "resources/db/queries/users.sql"]
struct DbUsers {}

pub async fn find_by_claims(pool: &Pool<Sqlite>, claims: &Claims) -> anyhow::Result<Option<User>> {
    let uuid = Uuid::from_str(&claims.sub)?;
    let user = DbUsers::fetch_user_by_id::<_, User>(pool, params!(uuid)).await?;
    Ok(user)
}

pub async fn find_by_user_id(pool: &Pool<Sqlite>, user_id: &Uuid) -> anyhow::Result<Option<User>> {
    let user = DbUsers::fetch_user_by_id::<_, User>(pool, params!(user_id)).await?;
    Ok(user)
}

#[tracing::instrument(skip(pool, user))]
pub async fn store(pool: &Pool<Sqlite>, user: User) -> anyhow::Result<User> {
    let uid = user.user_id;
    let email = user.email.to_lowercase();

    // There are 3 cases to consider:
    //
    // 1. Given `user_id` already exsists in database. The request then is just a
    //    plain update of user's fundamental properties (email, picture, ...)
    // 2. Given `user_id` does not exist in database but the `email` does. It means
    //    that upstream user record has been regenerated and came back as "new" user
    //    (user_id is different). In this case plain insert is impossible - uniqueness
    //    error on email will be thrown. Instead, user_id along with fundamental user's
    //    data needs to be updated for given email in database.
    // 3. There is no user with given `user_id` or `email`. Simplest case - new user
    //    record needs to be inserted.

    if find_by_user_id(pool, &uid).await?.is_some() {
        DbUsers::update_user_data(pool, params![email, &user.name, &user.picture, uid]).await?;
    } else {
        DbUsers::upsert_user(pool, params![uid, email, &user.name, &user.picture]).await?;
    }

    match find_by_user_id(pool, &uid).await {
        Ok(user) => Ok(user.unwrap()),
        Err(e) => {
            tracing::error!("User stored but not found. This should not happen: {}", e);
            bail!(DbError::UserNotFound)
        }
    }
}

pub async fn user_update(
    State(pool): State<SqlitePool>,
    Form(user): Form<User>,
) -> Result<Json<User>, AuthError> {
    tracing::info!("Updating user's profile... {:?}", user);

    let user = store(&pool, user).await.map_err(|e| {
        tracing::error!("Could not store user's profile: {}", e);
        AuthError::JWKSFetchError
    })?;

    Ok(Json(user))
}

pub async fn user_identity(user: User) -> Result<Json<User>, StatusCode> {
    tracing::info!("Fetching user's profile");
    Ok(Json(user))
}
