use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handlers;
mod routes;
mod models;

use handlers::root;
use routes::{ hello::hello_routes, json::json_routes };

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", axum::routing::get(root::root))
        .merge(hello_routes())
        .merge(json_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
