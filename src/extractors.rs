use alcoholic_jwt::JWKS;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
    Extension, TypedHeader,
};
use headers::{authorization::Bearer, Authorization};
use sqlx::PgPool;

use crate::{
    errors::AuthError,
    jwt::{self, Claims},
};

struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) ->  Result<Self, Self::Rejection>  {
        use axum::RequestPartsExt;
        let Extension((authority, jwks)) = parts.extract::<Extension<(String, JWKS)>>()
            .await
            .map_err(|_| AuthError::JWKSFetchError)?;

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError::InvalidToken)?;

        let claims = jwt::validate_token(bearer.token(), jwks, authority).await?;
        Ok(claims)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}


/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
