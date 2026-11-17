use std::net::SocketAddr;

use axum::{Router, routing::get_service};
use tower_http::services::ServeDir;

pub async fn start_axum_server() {
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .nest_service("/maps", get_service(ServeDir::new("./maps")));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://127.0.0.1:3000");
    
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");
    let _ = axum::serve(listener, app).await;
}