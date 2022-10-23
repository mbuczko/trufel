use std::str::FromStr;

use crate::errors::DbError;
use crate::jwt::Claims;

use anyhow::bail;
use hugsqlx::{params, HugSql};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

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

#[derive(HugSql)]
#[queries = "resources/db/queries/users.sql"]
struct Users {}

pub async fn find_by_claims(
    pool: &Pool<Postgres>,
    claims: &Claims,
) -> anyhow::Result<Option<User>> {
    let uuid = Uuid::from_str(&claims.sub)?;
    let user = Users::fetch_user_by_id::<User>(pool, params![uuid]).await?;

    Ok(user)
}

#[tracing::instrument(name = "Updating user's data in DB", skip(pool, claims))]
pub async fn store(pool: &Pool<Postgres>, claims: Claims) -> anyhow::Result<User> {
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
        Users::update_user_data(pool, params![email, &claims.name, &claims.picture, uuid]).await?;
    } else {
        Users::upsert_user(pool, params![uuid, email, &claims.name, &claims.picture]).await?;
    }

    match find_by_claims(pool, &claims).await {
        Ok(user) => Ok(user.unwrap()),
        Err(e) => {
            tracing::error!("User stored but not found. This should not happen: {}", e);
            bail!(DbError::UserNotFound)
        }
    }
}
