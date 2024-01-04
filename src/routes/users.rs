use anyhow::bail;
use axum::{extract::State, http::StatusCode, Json};
use hugsqlx::{params, HugSqlx};
use serde::Serialize;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::str::FromStr;
use uuid::Uuid;

use crate::errors::AuthError;
use crate::errors::DbError;
use crate::jwt::Claims;

#[derive(HugSqlx)]
#[queries = "resources/db/queries/users.sql"]
struct DbUsers {}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct UserProfile {
    pub login: String,
}

pub async fn find_by_claims(pool: &Pool<Sqlite>, claims: &Claims) -> anyhow::Result<Option<User>> {
    let uuid = Uuid::from_str(&claims.sub)?;
    let user = DbUsers::fetch_user_by_id::<_, User>(pool, params!(uuid)).await?;
    Ok(user)
}

#[tracing::instrument(skip(pool, claims))]
pub async fn store(pool: &Pool<Sqlite>, claims: Claims) -> anyhow::Result<User> {
    assert!(claims.email.is_some());
    assert!(claims.name.is_some());

    let uuid = Uuid::from_str(&claims.sub)?;
    let email = claims.email.as_ref().unwrap().to_lowercase();

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

    if find_by_claims(pool, &claims).await?.is_some() {
        DbUsers::update_user_data(pool, params![email, &claims.name, &claims.picture, uuid])
            .await?;
    } else {
        DbUsers::upsert_user(pool, params![uuid, email, &claims.name, &claims.picture]).await?;
    }

    match find_by_claims(pool, &claims).await {
        Ok(user) => Ok(user.unwrap()),
        Err(e) => {
            tracing::error!("User stored but not found. This should not happen: {}", e);
            bail!(DbError::UserNotFound)
        }
    }
}

pub async fn user_update(
    claims: Claims,
    State(pool): State<SqlitePool>,
) -> Result<Json<User>, AuthError> {
    tracing::info!("Updating user's profile...");

    let user = store(&pool, claims).await.map_err(|e| {
        tracing::error!("Could not store user's profile: {}", e);
        AuthError::JWKSFetchError
    })?;

    Ok(Json(user))
}

pub async fn user_identity(user: User) -> Result<Json<User>, StatusCode> {
    tracing::info!("Fetching user's profile");
    Ok(Json(user))
}

