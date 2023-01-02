use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::{Pool, Postgres};
use subtle_encoding::hex;

use crate::{jwt::Claims, user};

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct PusherUserData {
    pub name: String,
}

#[derive(Serialize, Debug)]
pub struct PusherAuth {
    pub auth: String,
    pub user_data: PusherUserData,
}

#[derive(Deserialize, Debug)]
pub struct AuthRequestPayload {
    pub socket_id: String,
    pub channel_name: String,
}

pub async fn auth_by_claims(
    socket_id: String,
    channel_name: String,
    pool: &Pool<Postgres>,
    claims: &Claims,
) -> anyhow::Result<Option<PusherAuth>> {
    if let Some(user) = user::find_by_claims(pool, claims).await? {
        let key = std::env::var("PUSHER_KEY").unwrap();
        let secret = std::env::var("PUSHER_SECRET").unwrap();

        // Generating authentication and authorization strings
        // https://pusher.com/docs/channels/library_auth_reference/auth-signatures/

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
        mac.update(format!("{socket_id}:{channel_name}").as_bytes());

        let result = hex::encode(mac.finalize().into_bytes());
        return Ok(Some(PusherAuth {
            auth: format!("{key}:{}", String::from_utf8(result).unwrap()),
            user_data: PusherUserData { name: user.name },
        }));
    }
    Ok(None)
}
