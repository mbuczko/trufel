use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    Extension,
};
use sqlx::SqlitePool;

use crate::{
    jwt::Claims,
    routes::users::{self, User}, errors::AuthError,
};

struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Sqlite>);

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        use axum::RequestPartsExt;

        let Extension(claims) = parts
            .extract::<Extension<Claims>>()
            .await
            .map_err(|_| AuthError::InvalidClaims)?;

        Ok(claims)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    SqlitePool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use axum::RequestPartsExt;
        let Extension(claims) = parts
            .extract::<Extension<Claims>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let pool = SqlitePool::from_ref(state);
        let user = users::find_by_claims(&pool, &claims)
            .await
            .map_err(|_| AuthError::InvalidClaims)?;

        Ok(user.unwrap())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    SqlitePool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = SqlitePool::from_ref(state);
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
