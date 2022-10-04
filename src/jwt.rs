use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};

use crate::errors::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdToken {
    pub email: String,
    pub name: String,
    pub picture: String,
    pub sub: String,
}

pub async fn fetch_jwks(authority: &str) -> Result<JWKS, AuthError> {
    let jwks = reqwest::get(format!("{}{}", authority, "protocol/openid-connect/certs"))
        .await
        .map_err(|_| AuthError::JWKSFetchError)?
        .json::<JWKS>()
        .await
        .map_err(|e| {
            log::error!("JWKS deserializing error: {}", e);
            AuthError::JWKSDeserializeError
        })?;

    Ok(jwks)
}

pub async fn validate_token(
    token: &str,
    jwks: JWKS,
    authority: String,
) -> Result<Claims, AuthError> {
    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = token_kid(token)
        .expect("Failed to decode token headers")
        .expect("No 'kid' claim present in token");

    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    match validate(token, jwk, validations) {
        Ok(jwt) => Ok(Claims {
            sub: jwt.claims["sub"].as_str().unwrap().to_string(),
            exp: jwt.claims["exp"].as_u64().unwrap() as usize,
        }),
        Err(e) => {
            println!("validation error: {:?}", e);
            Err(AuthError::JWTValidationError(e))
        }
    }
}
