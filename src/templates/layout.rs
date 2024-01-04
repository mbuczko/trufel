use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

pub async fn main() -> impl IntoResponse {
    HelloTemplate { name: "world" }
}
