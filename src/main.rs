mod db;
mod errors;
mod extractors;
mod jwt;
mod user;
mod vault;

use axum::{
    handler::Handler,
    http::{Method, StatusCode},
    routing::get,
    Extension, Json, Router,
};
use jwt::Claims;
use semver::Version;
use std::{net::SocketAddr, process::exit};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use user::User;
use vault::Vault;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "trufel=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = jwt::fetch_jwks(&authority).await?;

    let vault = match vault::init_vault("trufel", Version::parse(VERSION).unwrap()).await {
        Ok(vault) => vault,
        Err(_) => {
            eprintln!("Migration failed");
            exit(1);
        }
    };

    let app = Router::new()
        .route("/profile", get(user_profile.layer(CompressionLayer::new())))
        .layer(Extension((authority, jwks)))
        .layer(Extension(vault))
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT])
                .allow_origin(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn user_profile(claims: Claims, vault: Vault) -> Result<Json<User>, StatusCode> {
    match user::find_by_claims(&vault, &claims).await {
        Ok(some_user) => {
            if let Some(user) = some_user {
                Ok(Json(user))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            log::error!("Could not fetch user's profile: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
