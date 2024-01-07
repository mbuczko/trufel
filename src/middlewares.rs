use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::jwt::Claims;

pub async fn add_claim_details(
    claims: Option<Claims>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
