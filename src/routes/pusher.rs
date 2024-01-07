use axum::{extract::State, http::StatusCode, Form, Json};
use futures::TryFutureExt;
use hmac::{Hmac, Mac};
use pusher::PusherBuilder;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::SqlitePool;
use sqlx::{Pool, Sqlite};
use subtle_encoding::hex;

use crate::jwt::Claims;
use crate::models::user;

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
    pool: &Pool<Sqlite>,
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

//#[axum_macros::debug_handler]
pub async fn pusher_auth(
    claims: Claims,
    State(pool): State<SqlitePool>,
    Form(payload): Form<AuthRequestPayload>,
) -> Result<Json<PusherAuth>, StatusCode> {
    tracing::info!("Authenticating pusher connection...");
    match auth_by_claims(payload.socket_id, payload.channel_name, &pool, &claims).await {
        Ok(some_auth) => {
            if let Some(auth) = some_auth {
                Ok(Json(auth))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Could not authenticate pusher request: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn pusher_test() -> Result<(), StatusCode> {
    let key = std::env::var("PUSHER_KEY").unwrap();
    let secret = std::env::var("PUSHER_SECRET").unwrap();

    let pusher = PusherBuilder::new("trufel", &key, &secret)
        .host("pusher.rodzinks.pl")
        .finalize();

    pusher
        .trigger("private-chat-room", "message", "dupa")
        .map_err(|err| {
            tracing::error!(err = err, "Cannot trigger a message");
            StatusCode::BAD_REQUEST
        })
        .await?;

    Ok(())
}
