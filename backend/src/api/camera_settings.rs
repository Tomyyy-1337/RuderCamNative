use axum::http::StatusCode;

use crate::AppState;

pub async fn set_focal_length(
    state: AppState,
    focal_length: u8,
) -> StatusCode {
    state.camera_sender.send_modify(|interface| {
        interface.focal_length = focal_length;
    });
    StatusCode::OK
}