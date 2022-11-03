mod db;
mod errors;
mod extractors;
mod jwt;
mod telemetry;
mod user;
mod vault;

use axum::{
    extract::State,
    http::{Method, StatusCode},
    routing::{get, post},
    Extension, Json, Router,
};
use errors::AuthError;
use jwt::Claims;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use semver::Version;
use sqlx::PgPool;
use std::{net::SocketAddr, process::exit};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_log::LogTracer;
use user::User;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = jwt::fetch_jwks(&authority).await?;

    let vault = match vault::init_vault("trufel", Version::parse(VERSION).unwrap()).await {
        Ok(vault) => vault,
        Err(_) => {
            eprintln!("Migration failed");
            exit(1);
        }
    };

    telemetry::init_telemetry()?;

    let app = Router::with_state(vault.pool)
        .route("/@me", get(user_identity))
        .route("/user", post(user_update))
        .layer(CompressionLayer::new())
        .layer(Extension((authority, jwks)))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT])
                .allow_headers(vec![AUTHORIZATION, CONTENT_TYPE]),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(telemetry::make_span)
                .on_response(telemetry::emit_response_trace_with_id),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn user_update(claims: Claims, State(pool): State<PgPool>) -> Result<Json<User>, AuthError> {
    tracing::info!("Updating user's profile...");
    let user = user::store(&pool, claims).await.map_err(|e| {
        tracing::error!("Could not store user's profile: {}", e);
        AuthError::JWKSFetchError
    })?;
    Ok(Json(user))
}

async fn user_identity(
    claims: Claims,
    State(pool): State<PgPool>,
) -> Result<Json<User>, StatusCode> {
    match user::find_by_claims(&pool, &claims).await {
        Ok(some_user) => {
            if let Some(user) = some_user {
                Ok(Json(user))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Could not fetch user's profile: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
