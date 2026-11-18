use std::net::SocketAddr;

use axum::{Router, routing::get_service};
use tokio::{runtime::LocalOptions, sync::watch};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{hardware_interface::i2c_interface::I2cInterface, tasks::update_camera::update_camera_task};

mod api;
mod hardware_interface;
mod tasks;

lazy_static::lazy_static! {
    pub static ref I2C_INTERFACE: I2cInterface = I2cInterface::new();
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
            

            tokio::task::spawn_local(update_camera_task(camera_interface_receiver.clone()));


            let addr = SocketAddr::from(([0, 0, 0, 0], 8140));
            let app = Router::new()
                .route("/", axum::routing::get(|| async { "Hello, World!" }))
                .merge(shared_data::api::set_focal_length(api::camera_settings::set_focal_length_handler, camera_interface_sender.clone()))
                .merge(shared_data::api::set_hdr_active(api::camera_settings::set_hdr_active_handler, camera_interface_sender.clone()))
                .merge(shared_data::api::set_bitrate(api::camera_settings::set_bitrate_handler, camera_interface_sender.clone()))
                .merge(shared_data::api::set_focus_mode(api::camera_settings::set_focus_mode_handler, camera_interface_sender.clone()))
                .merge(shared_data::api::set_exposure_compensation(api::camera_settings::set_exposure_compensation_handler, camera_interface_sender.clone()))
                .merge(shared_data::api::set_metering_mode(api::camera_settings::set_metering_mode_handler, camera_interface_sender))
                .route("/ws", axum::routing::get(move |ws: axum::extract::ws::WebSocketUpgrade| api::socket::ws_handler(ws, 
                    camera_interface_receiver
                )))
                .nest_service("/maps", get_service(ServeDir::new("./maps")))
                .layer(CorsLayer::permissive());
            
            let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");
            println!("Server running on http://127.0.0.1:8140");
            let _ = axum::serve(listener, app).await;
        });
}
