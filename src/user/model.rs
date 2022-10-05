use std::str::FromStr;

use crate::jwt::Claims;
use crate::vault::Vault;

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}

#[derive(Serialize, Debug)]
pub struct UserProfile {
    pub login: String,
}

pub async fn find_by_claims(vault: &Vault, claims: &Claims) -> anyhow::Result<Option<User>> {
    let mut conn = vault.pool.acquire().await?;
    let user = sqlx::query_as::<_, User>("SELECT email, name, picture FROM users WHERE sub = ?1")
        .bind(&claims.sub)
        .fetch_optional(&mut conn)
        .await?;

    Ok(user)
}

pub async fn store(vault: &Vault, claims: Claims) -> anyhow::Result<Uuid> {
    let mut conn = vault.pool.acquire().await?;
    let uuid = Uuid::from_str(&claims.sub)?;

    assert!(claims.name.is_some());

    sqlx::query(r#"
            INSERT INTO users(user_id, email, idp_name, idp_picture) VALUES($1, $2, $3, $4)
            ON CONFLICT (user_id) DO UPDATE SET email=EXCLUDED.email, idp_name=EXCLUDED.idp_name, idp_picture=EXCLUDED.idp_picture
            "#
    )
    .bind(uuid)
    .bind(claims.email)
    .bind(claims.name)
    .bind(claims.picture)
    .execute(&mut conn)
    .await?;

    Ok(uuid)
}
