#![feature(str_split_remainder)]

mod db;
mod errors;
mod extractors;
mod jwt;
mod middlewares;
mod models;
mod routes;
mod sentry;
mod telemetry;

use axum::{
    http::{header, Method},
    middleware,
    routing::{get, post},
    Extension, Router,
};
use semver::Version;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_log::LogTracer;

use routes::categories;
use routes::pusher;
use routes::users;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    // OpenTelemetry and Sentry integration
    telemetry::init_telemetry()?;
    let _sentry = sentry::init_sentry();

    // JWT and OIDC integration
    let jwks = jwt::fetch_jwks().await?;

    // SQLite connection pre-initialized with a migration scripts if needed
    let pool = db::init_pool()
        .await
        .expect("Could not connect to database");

    db::migrate(&pool, Version::parse(env!("CARGO_PKG_VERSION")).unwrap()).await?;

    let serve_dir = ServeDir::new("dist/assets");
    let app = Router::new()
        .route("/@me", get(users::user_identity))
        .route("/user", post(users::user_update))
        .route("/categories", get(categories::categories))
        .route("/categories", post(categories::add_category))
        // .route("/components", post(components::fetch_components))
        .route("/pusher/auth", post(pusher::pusher_auth))
        .route("/pusher/test", get(pusher::pusher_test))
        .route_layer(middleware::from_fn(middlewares::add_claim_details))
        .nest_service("/assets", serve_dir.clone())
        .with_state(pool)
        .layer(CompressionLayer::new())
        .layer(Extension(jwks))
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
