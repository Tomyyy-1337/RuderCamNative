use axum::http::StatusCode;
use shared_data::{CameraInterface, FocusMode, Metering};
use tokio::sync::watch;

pub async fn set_focal_length_handler(
    state: watch::Sender<CameraInterface>,
    focal_length: u8,
) -> StatusCode {
    state.send_modify(|interface| {
        interface.focal_length = focal_length;
    });
    StatusCode::OK
}

pub async fn set_exposure_compensation_handler(
    state: watch::Sender<CameraInterface>,
    exposure_compensation: f32,
) -> StatusCode {
    state.send_modify(|interface| {
        interface.exposure_compenstion = exposure_compensation;
    });
    println!("Exposure compensation set to {}", exposure_compensation);
    StatusCode::OK
}

pub async fn set_hdr_active_handler(
    state: watch::Sender<CameraInterface>,
    hdr_active: bool,
) -> StatusCode {
    state.send_modify(|interface| {
        interface.hdr_enabled = hdr_active;
    });
    StatusCode::OK
}

pub async fn set_bitrate_handler(
    state: watch::Sender<CameraInterface>,
    bitrate: u32,
) -> StatusCode {
    state.send_modify(|interface| {
        interface.bitrate = bitrate;
    });
    StatusCode::OK
}

pub async fn set_focus_mode_handler(
    state: watch::Sender<CameraInterface>,
    focus_mode: FocusMode
 ) -> StatusCode {
    state.send_modify(|interface| {
        interface.focus_mode = focus_mode;
    });
    StatusCode::OK
}

pub async fn set_metering_mode_handler(
    state: watch::Sender<CameraInterface>,
    metering_mode: Metering
) -> StatusCode {
    state.send_modify(|interface| {
        interface.metering_mode = metering_mode;
    });
    StatusCode::OK
}

