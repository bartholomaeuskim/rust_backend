use axum::{ routing::post, Router, Json };
use crate::models::request::Input;
use crate::models::response::Response;

async fn json_handler(Json(payload): Json<Input>) -> Json<Response> {
    let response = Response {
        message: format!("Hello, {}!", payload.name),
    };
    Json(response)
}

pub fn json_routes() -> Router {
    Router::new().route("/json", post(json_handler))
}