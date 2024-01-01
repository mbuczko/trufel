mod db;
mod errors;
mod extractors;
mod jwt;
mod pusher;
mod telemetry;
mod user;
mod sentry;

use ::pusher::PusherBuilder;
use axum::{
    extract::State,
    http::{Method, StatusCode, header},
    routing::{get, post},
    Extension, Form, Json, Router,
};
use errors::AuthError;
use futures::TryFutureExt;
use jwt::Claims;
use semver::Version;
use tracing_log::LogTracer;
use sqlx::SqlitePool;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use user::User;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LogTracer::init().expect("Failed to set logger");
    telemetry::init_telemetry()?;

    let _sentry = sentry::init_sentry();
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = jwt::fetch_jwks(&authority).await?;

    let pool = db::init_pool()
        .await
        .expect("Could not connect to database");

    // apply migrations to keep database schema up to date
    db::migrate(&pool, Version::parse(env!("CARGO_PKG_VERSION")).unwrap()).await?;

    let app = Router::new()
        .route("/@me", get(user_identity))
        .route("/user", post(user_update))
        .route("/pusher/auth", post(pusher_auth))
        .route("/pusher/test", get(pusher_test))
        .with_state(pool)
        .layer(CompressionLayer::new())
        .layer(Extension((authority, jwks)))
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

async fn user_update(claims: Claims, State(pool): State<SqlitePool>) -> Result<Json<User>, AuthError> {
    tracing::info!("Updating user's profile...");
    let user = user::store(&pool, claims).await.map_err(|e| {
        tracing::error!("Could not store user's profile: {}", e);
        AuthError::JWKSFetchError
    })?;
    Ok(Json(user))
}

async fn user_identity(
    claims: Claims,
    State(pool): State<SqlitePool>,
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

//#[axum_macros::debug_handler]
async fn pusher_auth(
    claims: Claims,
    State(pool): State<SqlitePool>,
    Form(payload): Form<pusher::AuthRequestPayload>,
) -> Result<Json<pusher::PusherAuth>, StatusCode> {
    tracing::info!("Authenticating pusher connection...");
    match pusher::auth_by_claims(payload.socket_id, payload.channel_name, &pool, &claims).await {
        Ok(some_auth) => {
            if let Some(auth) = some_auth {
                Ok(Json(auth))
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            tracing::error!("Could not authenticate pusher request: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn pusher_test() -> Result<(), StatusCode> {
    let key = std::env::var("PUSHER_KEY").unwrap();
    let secret = std::env::var("PUSHER_SECRET").unwrap();

    let pusher = PusherBuilder::new("trufel", &key, &secret)
        .host("pusher.rodzinks.pl")
        .finalize();

    pusher
        .trigger("private-chat-room", "message", "dupa")
        .map_err(|err| {
            tracing::error!(err = err, "Cannot trigger a message");
            StatusCode::BAD_REQUEST
        })
        .await?;

    Ok(())
}
