use axum::routing::get;
use axum::Router;

async fn hello() -> &'static str {
    "Hello World!"
}

pub fn hello_routes() -> Router {
    Router::new().route("/hello", get(hello))
}