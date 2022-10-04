use alcoholic_jwt::JWKS;
use axum::{
    http::StatusCode,
    Extension, async_trait, extract::{FromRequest, RequestParts}, TypedHeader,
};
use headers::{Authorization, authorization::Bearer};
use serde_json::{Value, json};

use crate::{jwt::{Claims, self}, errors::AuthError, vault::Vault};


#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: axum::body::HttpBody + Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> anyhow::Result<Self, Self::Rejection> {
        let Extension((authority, jwks)) = Extension::<(String, JWKS)>::from_request(req)
            .await
            .map_err(|_| AuthError::JWKSFetchError)?;

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;

        let claims = jwt::validate_token(bearer.token(), jwks, authority).await?;
        Ok(claims)
    }
}

#[async_trait]
impl<B> FromRequest<B> for Vault
where
    B: axum::body::HttpBody + Send,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> anyhow::Result<Self, Self::Rejection> {
        let Extension(vault) = Extension::<Vault>::from_request(req).await.map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(json!({ "error": format!("Cannot open db connections pool: {}", err) }))
            )
        })?;
        Ok(vault)
    }
}
