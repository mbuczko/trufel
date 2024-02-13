use anyhow::bail;
use hugsqlx::{params, HugSqlx};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::str::FromStr;
use uuid::Uuid;

use crate::{errors::InternalError, jwt::Claims};

#[derive(HugSqlx)]
#[queries = "resources/db/queries/users.sql"]
struct DbUsers {}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: Uuid,
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

pub async fn find_by_user_id(pool: &Pool<Sqlite>, user_id: &Uuid) -> anyhow::Result<Option<User>> {
    let user = DbUsers::fetch_user_by_id::<_, User>(pool, params!(user_id)).await?;
    Ok(user)
}

#[tracing::instrument(skip(pool, user))]
pub async fn store(pool: &Pool<Sqlite>, user: User) -> anyhow::Result<User> {
    let uid = user.id;
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
            tracing::error!(error = ?e, "User stored but not found. This should not happen");
            bail!(InternalError::UserUpdate)
        }
    }
}
