use std::str::FromStr;

use crate::errors::DbError;
use crate::jwt::Claims;
use crate::vault::Vault;

use anyhow::bail;
use serde::Serialize;
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

pub async fn find_by_claims(vault: &Vault, claims: &Claims) -> anyhow::Result<Option<User>> {
    let mut conn = vault.pool.acquire().await?;
    let uuid = Uuid::from_str(&claims.sub)?;
    let user = sqlx::query_as::<_, User>(
        "SELECT user_id, email, name, picture FROM users WHERE user_id = $1",
    )
    .bind(uuid)
    .fetch_optional(&mut conn)
    .await?;

    Ok(user)
}

#[tracing::instrument(
    name = "Updating user's data in DB",
    skip(vault, claims))
]
pub async fn store(vault: &Vault, claims: Claims) -> anyhow::Result<User> {
    let mut conn = vault.pool.acquire().await?;

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

    if find_by_claims(vault, &claims).await?.is_some() {
        sqlx::query(
            r#"
            UPDATE users SET email=$1, name=$2, picture=$3
            WHERE user_id=$4
        "#,
        )
        .bind(email)
        .bind(&claims.name)
        .bind(&claims.picture)
        .bind(uuid)
        .execute(&mut conn)
        .await?;
    } else {
        sqlx::query(
            r#"
            INSERT INTO users(user_id, email, name, picture) VALUES($1, $2, $3, $4)
            ON CONFLICT (email) DO UPDATE
            SET user_id=EXCLUDED.user_id, name=EXCLUDED.name, picture=EXCLUDED.picture
            "#,
        )
        .bind(uuid)
        .bind(email)
        .bind(&claims.name)
        .bind(&claims.picture)
        .execute(&mut conn)
        .await?;
    }

    match find_by_claims(vault, &claims).await {
        Ok(user) => Ok(user.unwrap()),
        Err(e) => {
            tracing::error!("User stored but not found. This should not happen: {}", e);
            bail!(DbError::UserNotFound)
        }
    }
}
