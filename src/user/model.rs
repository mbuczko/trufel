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
    let user = sqlx::query_as::<_, User>("SELECT user_id, email, name, picture FROM users WHERE user_id = $1")
        .bind(uuid)
        .fetch_optional(&mut conn)
        .await?;

    Ok(user)
}

pub async fn store(vault: &Vault, claims: Claims) -> anyhow::Result<User> {
    let mut conn = vault.pool.acquire().await?;
    let uuid = Uuid::from_str(&claims.sub)?;

    assert!(claims.email.is_some());
    assert!(claims.name.is_some());

    sqlx::query(r#"
            INSERT INTO users(user_id, email, name, picture) VALUES($1, $2, $3, $4)
            ON CONFLICT (user_id) DO UPDATE SET email=EXCLUDED.email, name=EXCLUDED.name, picture=EXCLUDED.picture
            "#
    )
    .bind(uuid)
    .bind(&claims.email)
    .bind(&claims.name)
    .bind(&claims.picture)
    .execute(&mut conn)
    .await?;

    match find_by_claims(vault, &claims).await {
        Ok(user) => Ok(user.unwrap()),
        Err(e) => {
            log::error!("User stored but not found. This should not happen: {}", e);
            bail!(DbError::UserNotFound)
        }
    }
}
