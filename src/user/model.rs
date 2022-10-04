use crate::jwt::Claims;
use crate::jwt::IdToken;
use crate::vault::Vault;

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct User {
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

pub async fn store(vault: &Vault, id_token: IdToken) -> anyhow::Result<Uuid> {
    let mut conn = vault.pool.acquire().await?;
    let uuid = Uuid::new_v4();

    log::debug!("UUID: {}", uuid);

    sqlx::query(r#"
            INSERT INTO users(user_id, sub, email, name, picture) VALUES($1, $2, $3, $4, $5)
            ON CONFLICT (sub) DO UPDATE SET email=EXCLUDED.email, name=EXCLUDED.name, picture=EXCLUDED.picture
            "#
    )
    .bind(&uuid)
    .bind(id_token.sub)
    .bind(id_token.email)
    .bind(id_token.name)
    .bind(id_token.picture)
    .execute(&mut conn)
    .await?;

    Ok(uuid)
}
