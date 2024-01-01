mod db;
mod errors;
mod extractors;
mod jwt;
mod telemetry;
mod user;

use alcoholic_jwt::JWKS;
use axum::{
    handler::Handler,
    http::{Method, StatusCode, header},
    routing::{get, post}, Json, Router,
};
use extractors::DatabaseConnection;
use jwt::Claims;
use semver::Version;
use sqlx::SqlitePool;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_log::LogTracer;
use user::User;

#[derive(Clone)]
struct AppState {
    auth_config: (String, JWKS),
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    telemetry::init_telemetry()?;

    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = jwt::fetch_jwks(&authority).await?;

    let pool = db::init_pool()
        .await
        .expect("Could not connect to database");

    // apply migrations to keep database schema up to date
    db::migrate(&pool, Version::parse(env!("CARGO_PKG_VERSION")).unwrap()).await?;

    let app = Router::new()
        .route("/@me", get(user_identity.layer(CompressionLayer::new())))
        .route("/user", post(user_update))
        .with_state(AppState { auth_config: (authority, jwks), pool})
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT])
                .allow_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE]),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(telemetry::make_span)
                .on_response(telemetry::emit_response_trace_with_id),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030")
        .await
        .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn user_update(claims: Claims, db: DatabaseConnection) -> Result<Json<User>, StatusCode> {
    tracing::info!("Updating user's profile...");
    let user = user::store(db, claims).await.map_err(|e| {
        tracing::error!("Could not store user's profile: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(user))
}

async fn user_identity(claims: Claims, mut db: DatabaseConnection) -> Result<Json<User>, StatusCode> {
    match user::find_by_claims(&mut db, &claims).await {
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
