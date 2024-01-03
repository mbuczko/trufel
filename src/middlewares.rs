use alcoholic_jwt::JWKS;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::jwt;

pub async fn add_claim_details(
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let extensions = request.extensions();

    if let Some(TypedHeader(Authorization(bearer))) =
        extensions.get::<TypedHeader<Authorization<Bearer>>>()
    {
        if let Some(Extension(jwks)) = extensions.get::<Extension<JWKS>>() {
            if let Ok(claims) = jwt::validate_token(bearer.token(), jwks).await {
                request.extensions_mut().insert(claims);
            }
        }
    }
    Ok(next.run(request).await)
}
