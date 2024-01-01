use axum::body::Body;
use axum::extract::{MatchedPath, OriginalUri};
use axum::http::uri::Scheme;
use axum::http::{Request, Response, self, header, Method};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::{RandomIdGenerator, IdGenerator};
use opentelemetry::trace::{TraceId, SpanContext, SpanId, TraceContextExt};
use percent_encoding::percent_decode_str;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use std::borrow::{Cow, Borrow};
use std::time::Duration;
use tracing::field;
use tracing_subscriber::{layer::SubscriberExt, Registry};

pub fn init_telemetry() -> anyhow::Result<()> {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    // let exporter = opentelemetry_otlp::new_exporter()
    //     .tonic()
    //     .with_endpoint("http://localhost:4317")
    //     .with_timeout(Duration::from_secs(3));

    // let tempo_tracer = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     .with_exporter(exporter)
    //     .with_trace_config(trace::config().with_resource(Resource::new(vec![
    //         KeyValue::new("service.name", std::env::var("SERVICE_NAME").unwrap()),
    //         KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
    //     ])))
    //     .install_batch(opentelemetry::runtime::Tokio)
    //     .expect("Error - Failed to create tracer.");

    let subscriber = Registry::default()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "trufel=debug,tower_http=debug".into()),
        ))
        // .with(tracing_opentelemetry::layer().with_tracer(tempo_tracer))
        .with(tracing_subscriber::fmt::layer());

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

/// Internal helper for [`tower_http::trace::TraceLayer`] to create
/// [`tracing::Span`]s around a request.
pub fn make_span(request: &Request<Body>) -> tracing::Span {
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

    let (sentry_transaction_id, sentry_trace_id) = sentry_baggage(request);
    let (remote_context, trace_id) =
        create_context_with_trace(extract_remote_context(request.headers()), sentry_trace_id);

    let name = format!("{http_method} {http_route}");
    let span = tracing::info_span!(
        "http",
        trace_id = %trace_id,
        http.host = %host,
        http.scheme = %scheme,
        http.method = %http_method,
        http.route = %http_route,
        http.target = %http_target,
        http.user_agent = %user_agent,
        http.status_code = tracing::field::Empty,
        otel.kind = %"server",
        otel.name = %name,
    );

    sentry::configure_scope(|scope| {
        scope.set_transaction(sentry_transaction_id.as_ref().map(|tid| tid.borrow()));
    });

    span.set_parent(remote_context);
    span
}

pub fn emit_response_trace_with_id(response: &Response<Body>, latency: Duration, span: &tracing::Span) {
    let http_status = response.status().as_u16();
    let status_code = &field::display(http_status);

    span.record("request_duration", &field::display(latency.as_micros()));
    span.record("status_code", status_code);
    span.record("http.status_code", status_code);
    span.record("otel.status_code", if http_status != 500 { "OK" } else { "ERROR" });
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

// If remote request has no span data the propagator defaults to an unsampled context
fn extract_remote_context(headers: &http::HeaderMap) -> opentelemetry::Context {
    struct HeaderExtractor<'a>(&'a http::HeaderMap);

    impl<'a> opentelemetry::propagation::Extractor for HeaderExtractor<'a> {
        fn get(&self, key: &str) -> Option<&str> {
            self.0.get(key).and_then(|value| value.to_str().ok())
        }

        fn keys(&self) -> Vec<&str> {
            self.0.keys().map(|value| value.as_str()).collect()
        }
    }
    let extractor = HeaderExtractor(headers);
    opentelemetry::global::get_text_map_propagator(|propagator| propagator.extract(&extractor))
}

fn create_context_with_trace(
    remote_context: opentelemetry::Context,
    sentry_trace_id: Option<Cow<str>>,
) -> (opentelemetry::Context, TraceId) {
    if !remote_context.span().span_context().is_valid() {
        // create a fake remote context but with a fresh new trace_id
        let trace_id = sentry_trace_id
            .map(|id| TraceId::from_hex(id.borrow()))
            .unwrap_or_else(|| Ok(RandomIdGenerator::default().new_trace_id()))
            .unwrap();
        let new_span_context = SpanContext::new(
            trace_id,
            SpanId::INVALID,
            remote_context.span().span_context().trace_flags(),
            false,
            remote_context.span().span_context().trace_state().clone(),
        );
        (
            remote_context.with_remote_span_context(new_span_context),
            trace_id,
        )
    } else {
        let remote_span = remote_context.span();
        let span_context = remote_span.span_context();
        let trace_id = span_context.trace_id();

        (remote_context, trace_id)
    }
}

fn sentry_baggage(request: &Request<Body>) -> (Option<Cow<str>>, Option<Cow<str>>) {
    // Sample baggage header looks like this:
    //
    // baggage: sentry-transaction=fetchMeta,
    //          sentry-public_key=0490a2c285ff494b91d9811c2f9bbf6d,
    //          sentry-trace_id=c33d136d19a74b0686dcfa3e9ab74ea9,
    //          sentry-sample_rate=1

    let mut transaction_id = None;
    let mut trace_id = None;

    if let Some(baggage) = request.headers().get("baggage") {
        baggage
            .to_str()
            .unwrap()
            .split(',')
            .flat_map(|s| s.split('='))
            .collect::<Vec<_>>()
            .chunks(2)
            .for_each(|slice| {
                if slice[0] == "sentry-transaction" {
                    transaction_id =
                        Some(percent_decode_str(slice[1].trim()).decode_utf8().unwrap());
                } else if slice[0] == "sentry-trace_id" {
                    trace_id = Some(Cow::from(slice[1]));
                }
            });
    };
    (transaction_id, trace_id)
}
