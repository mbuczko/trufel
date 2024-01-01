use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{headers, TypedHeader};
use headers::{authorization::Bearer, Authorization};
use serde_json::{json, Value};

use crate::{
    errors::AuthError,
    jwt::{self, Claims}, AppState,
};

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> anyhow::Result<Self, Self::Rejection> {
        let (authority, jwks) = AppState::from_ref(state).auth_config;

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let claims = jwt::validate_token(bearer.token(), jwks, authority).await?;
        Ok(claims)
    }
}

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Sqlite>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &S,
    ) -> anyhow::Result<Self, Self::Rejection> {
        let pool = AppState::from_ref(state).pool;
        let conn = pool.acquire().await.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(json!({
                    "error": format!("Cannot open db connections pool: {}", err)
                })),
            )
        })?;
        Ok(Self(conn))
    }
}
