mod db;
mod errors;
mod extractors;
mod jwt;
mod telemetry;
mod user;
mod vault;

use axum::{
    handler::Handler,
    http::{Method, StatusCode},
    routing::{get, post},
    Extension, Json, Router,
};
use hugsql::HugSql;
use jwt::Claims;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use semver::Version;
use std::{net::SocketAddr, process::exit};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_log::LogTracer;
use user::{User};
use vault::Vault;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(HugSql)]
#[queries = "resources/db/queries/"]
struct Db(User);

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

    let app = Router::new()
        .route("/@me", get(user_identity.layer(CompressionLayer::new())))
        .route("/user", post(user_update))
        .route("/test", get(user_test))
        .layer(Extension(vault))
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


async fn user_test(_claims: Claims, vault: Vault) -> Result<String, StatusCode> {
    tracing::info!("Updating user's profile...");

    let conn = vault.pool;
    Db::fetch_user_by_id(&conn, &[123]);

    Ok(String::from("Ok"))
}

async fn user_update(claims: Claims, vault: Vault) -> Result<Json<User>, StatusCode> {
    tracing::info!("Updating user's profile...");
    let user = user::store(&vault, claims).await.map_err(|e| {
        tracing::error!("Could not store user's profile: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(user))
}

async fn user_identity(claims: Claims, vault: Vault) -> Result<Json<User>, StatusCode> {
    match user::find_by_claims(&vault, &claims).await {
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
