use sentry::ClientInitGuard;
use std::env;

/// Initializes sentry - an exception sink
pub fn init_sentry() -> ClientInitGuard {
    let guard = sentry::init((
        "https://eef8de1087434d238e85ffcf3dd723b9@sentry.rodzinks.pl/3",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            // attach_stacktrace: true,
            ..Default::default()
        },
    ));
    sentry::configure_scope(|scope| {
        scope.set_tag("service_name", env::var("SERVICE_NAME").unwrap());
        // scope.set_tag("build_sha", build_sha.to_string());
    });
    guard
}
