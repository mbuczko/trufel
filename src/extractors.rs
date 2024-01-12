use alcoholic_jwt::JWKS;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    Extension,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sqlx::SqlitePool;

use crate::{
    errors::AuthError,
    jwt::{self, Claims},
    models::user::{self, User},
};

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        use axum::RequestPartsExt;
        let Extension(jwks) = parts
            .extract::<Extension<JWKS>>()
            .await
            .map_err(|_| AuthError::JWKSFetchError)?;

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let claims = jwt::validate_token(bearer.token(), &jwks).await?;
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
        let user = user::find_by_claims(&pool, &claims)
            .await
            .map_err(|_| AuthError::InvalidClaims)?;

        Ok(user.unwrap())
    }
}

// #[async_trait]
// impl<S> FromRequestParts<S> for DatabaseConnection
// where
//     SqlitePool: FromRef<S>,
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let pool = SqlitePool::from_ref(state);
//         let conn = pool.acquire().await.map_err(internal_error)?;
//         Ok(Self(conn))
//     }
// }

// /// Utility function for mapping any error into a `500 Internal Server Error`
// /// response.
// fn internal_error<E>(err: E) -> (StatusCode, String)
// where
//     E: std::error::Error,
// {
//     (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
// }
