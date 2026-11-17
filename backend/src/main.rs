use std::net::SocketAddr;

use axum::{Router, routing::get_service};
use shared_data::CameraInterface;
use tokio::{runtime::LocalOptions, sync::watch};
use tower_http::{cors::CorsLayer, services::ServeDir};

mod api;

#[derive(Clone)]
struct AppState {
    camera_sender: watch::Sender<CameraInterface>,
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .max_blocking_threads(4)
        .enable_all()
        .build_local(LocalOptions::default())
        .unwrap()
        .block_on(async {

            let camera_interface_initial = shared_data::CameraInterface::new();
            
            let (camera_interface_sender, camera_interface_receiver) = watch::channel(camera_interface_initial);
            
            let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
            let app = Router::new()
                .route("/", axum::routing::get(|| async { "Hello, World!" }))
                .merge(shared_data::api::set_focal_length(api::camera_settings::set_focal_length))
                .nest_service("/maps", get_service(ServeDir::new("./maps")))
                .with_state(AppState {
                    camera_sender: camera_interface_sender,
                })
                .layer(CorsLayer::permissive());
            
            let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");
            println!("Server running on http://127.0.0.1:3000");
            let _ = axum::serve(listener, app).await;
        });
}
