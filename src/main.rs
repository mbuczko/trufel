mod db;
mod errors;
mod extractors;
mod jwt;
mod user;
mod vault;

use axum::{
    handler::Handler,
    http::{Method, StatusCode},
    routing::{get, post},
    Extension, Json, Router,
};
use jwt::Claims;
use opentelemetry::{sdk::{trace, Resource, propagation::TraceContextPropagator}, global};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use semver::Version;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use uuid::Uuid;
use std::{collections::HashMap, net::SocketAddr, process::exit};
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
    //let stdout_tracer = stdout::new_pipeline().install_simple();
    let aspecto_key = std::env::var("ASPECTO_API_KEY").unwrap();
    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint("https://otelcol.aspecto.io/v1/traces")
        .with_headers(HashMap::from([("Authorization".into(), aspecto_key)]));

    let aspecto_tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                std::env::var("SERVICE_NAME").unwrap()
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Error - Failed to create tracer.");

    let formatting_layer = BunyanFormattingLayer::new(
        std::env::var("SERVICE_NAME").unwrap(),
        std::io::stdout,
    );

    global::set_text_map_propagator(TraceContextPropagator::new());

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "trufel=debug,tower_http=debug".into()),
        ))
        .with(tracing_opentelemetry::layer().with_tracer(aspecto_tracer))
        .with(JsonStorageLayer)
        .with(formatting_layer)
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
        .route("/@me", get(user_identity.layer(CompressionLayer::new())))
        .route("/user", post(user_update))
        .layer(Extension(vault))
        .layer(Extension((authority, jwks)))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT])
                .allow_headers(vec![AUTHORIZATION, CONTENT_TYPE]),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[tracing::instrument(
    name = "Post sign-in/sign-up user's data reconciliation",
    skip(claims, vault),
    fields(
        request_id = %Uuid::new_v4()
    )
)]
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
            log::error!("Could not fetch user's profile: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
