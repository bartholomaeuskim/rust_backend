use axum::{
    routing::{ get, post },
    Router, Json,
};
use serde::{ Deserialize, Serialize };
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/json", post(json_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Welcome to Rust Backend!"
}

async fn hello() -> &'static str {
    "Hello World!"
}

#[derive(Serialize, Deserialize)]
struct Input {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn json_handler(Json(payload): Json<Input>) -> Json<Response> {
    let response = Response {
        message: format!("Hello, {}!", payload.name),
    };
    Json(response)
}