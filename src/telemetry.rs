use axum::body::{Body, BoxBody};
use axum::extract::{OriginalUri, MatchedPath};
use axum::http::{Request, Response};
use axum::http::uri::Scheme;
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use reqwest::{Method, header};
use tracing::{debug_span, Span, field};
use std::collections::HashMap;
use std::time::Duration;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub fn init_telemetry() -> anyhow::Result<()> {
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
                std::env::var("SERVICE_NAME").unwrap(),
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Error - Failed to create tracer.");

    let formatting_layer =
        BunyanFormattingLayer::new(std::env::var("SERVICE_NAME").unwrap(), std::io::stdout);

    let subscriber = Registry::default()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "trufel=debug,tower_http=debug".into()),
        ))
        .with(tracing_opentelemetry::layer().with_tracer(aspecto_tracer))
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

/// Internal helper for [`tower_http::trace::TraceLayer`] to create
/// [`tracing::Span`]s around a request.
pub fn make_span(request: &Request<Body>) -> Span {
    let http_route = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else if let Some(uri) = request.extensions().get::<OriginalUri>() {
        uri.0.path().to_owned()
    } else {
        request.uri().path().to_owned()
    };
    let host = request
        .headers()
        .get(header::HOST)
        .map_or("", |h| h.to_str().unwrap_or(""));
    let scheme = request
        .uri()
        .scheme()
        .map_or_else(|| "HTTP".into(), http_scheme);
    let http_method = http_method(request.method());
    let user_agent = request
        .headers()
        .get(header::USER_AGENT)
        .map_or("", |h| h.to_str().unwrap_or(""));
    let uri = if let Some(uri) = request.extensions().get::<OriginalUri>() {
        uri.0.clone()
    } else {
        request.uri().clone()
    };
    let http_target = uri
        .path_and_query()
        .map(|path_and_query| path_and_query.to_string())
        .unwrap_or_else(|| uri.path().to_owned());

    debug_span!(
        "http-request",
        request_duration = tracing::field::Empty,
        status_code = tracing::field::Empty,
        traceID = tracing::field::Empty,
        http.host = host,
        http.scheme = scheme,
        http.method = http_method,
        http.route = http_route,
        http.target = http_target,
        http.user_agent = user_agent,
        http.status_code = tracing::field::Empty,
        otel.kind = "server",

    )
}

/// Internal helper for [`tower_http::trace::TraceLayer`] to emit a structured [`tracing::Span`] with specific recorded fields.
///
/// Uses a `Loki`-friendly `traceID` that can correlate to `Tempo` distributed traces.
pub fn emit_response_trace_with_id(response: &Response<BoxBody>, latency: Duration, span: &Span) {
    // https://github.com/kube-rs/controller-rs/blob/b99ad0bfbf4ae75f03323bff2796572d4257bd96/src/telemetry.rs#L4-L8
    use opentelemetry::trace::TraceContextExt;
    use tracing_opentelemetry::OpenTelemetrySpanExt;
    let trace_id = span.context().span().span_context().trace_id().to_string();
    let status_code = &field::display(response.status().as_u16());

    span.record("traceID", &field::display(&trace_id));
    span.record("request_duration", &field::display(latency.as_micros()));
    span.record("status_code", status_code);
    span.record("http.status_code", status_code);

    tracing::debug!("response generated");
}

fn http_method(method: &Method) -> String {
    match method {
        &Method::CONNECT => "CONNECT".into(),
        &Method::DELETE => "DELETE".into(),
        &Method::GET => "GET".into(),
        &Method::HEAD => "HEAD".into(),
        &Method::OPTIONS => "OPTIONS".into(),
        &Method::PATCH => "PATCH".into(),
        &Method::POST => "POST".into(),
        &Method::PUT => "PUT".into(),
        &Method::TRACE => "TRACE".into(),
        other => other.to_string(),
    }
}

fn http_scheme(scheme: &Scheme) -> String {
    if scheme == &Scheme::HTTP {
        "http".into()
    } else if scheme == &Scheme::HTTPS {
        "https".into()
    } else {
        scheme.to_string()
    }
}
