use std::env;

use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

use crate::errors::AuthError;

lazy_static! {
   static ref AUTHORITY: String = env::var("AUTHORITY").expect("AUTHORITY must be set");
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
}

pub async fn fetch_jwks() -> Result<JWKS, AuthError> {
    let jwks = reqwest::get(format!("{}/{}", *AUTHORITY, "protocol/openid-connect/certs"))
        .await
        .map_err(|_| AuthError::JWKSFetchError)?
        .json::<JWKS>()
        .await
        .map_err(|e| {
            tracing::error!("JWKS deserializing error: {}", e);
            AuthError::JWKSDeserializeError
        })?;

    Ok(jwks)
}

pub async fn validate_token(
    token: &str,
    jwks: &JWKS,
) -> Result<Claims, AuthError> {
    let validations = vec![Validation::Issuer(AUTHORITY.to_string()), Validation::SubjectPresent];
    let kid = token_kid(token)
        .expect("Failed to decode token headers")
        .expect("No 'kid' claim present in token");

    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    match validate(token, jwk, validations) {
        Ok(jwt) => Ok(Claims {
            sub: jwt.claims["sub"].as_str().unwrap().to_string(),
            exp: jwt.claims["exp"].as_u64().unwrap() as usize,
            name: jwt.claims["name"].as_str().map(String::from),
            email: jwt.claims["email"].as_str().map(String::from),
            picture: jwt.claims["picture"].as_str().map(String::from),
        }),
        Err(e) => {
            println!("validation error: {:?}", e);
            Err(AuthError::JWTValidationError(e))
        }
    }
}
