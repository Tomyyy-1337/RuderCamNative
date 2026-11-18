use std::{str::FromStr, time::Duration};
use serde::Serialize;
use shared_data::{CameraInterface, FocusMode, Metering};

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder().build().unwrap();
}

/// Send updates to the camera when the camera interface changes
pub async fn update_camera_task(mut camera_receiver: tokio::sync::watch::Receiver<CameraInterface>) {    
    while let Ok(_) = camera_receiver.changed().await {
        let camera_interface = camera_receiver.borrow().clone();
        let mut retry_count = 0;
        while retry_count < 3 && let Err(_e) = send_to_camera(&camera_interface).await{
            retry_count += 1;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        if retry_count == 3 {
            println!("Failed to send camera update after 3 attempts");
        }
    }
}

pub async fn send_to_camera(camera_interface: &CameraInterface) -> Result<(), reqwest::Error> {
    let config = CameraConfig::from(camera_interface);

    let response = CLIENT
        .patch("http://127.0.0.1:9997/v3/config/paths/patch/stream")
        .json(&config)
        .timeout(Duration::from_secs(5))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        println!("API error: {status}: {body}");
    }
    Ok(())
}

fn get_roi(camera_interface: &CameraInterface) -> String {
    const BASE_FOCAL_LENGTH: f32 = 28.0;
    
    let crop = BASE_FOCAL_LENGTH / camera_interface.focal_length as f32;
    let offset = (1.0 - crop) / 2.0;

    format!("{offset:.6},{offset:.6},{crop:.6},{crop:.6}")
}
    

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct CameraConfig {
    pub rpiCameraHDR: bool,
    pub rpiCameraEV: f32,
    pub rpiCameraMetering: String,
    pub rpiCameraAfMode: String,
    pub rpiCameraROI: String,
    pub rpiCameraBitrate: u32,
}

impl From<&CameraInterface> for CameraConfig {
    fn from(camera: &CameraInterface) -> Self {
        CameraConfig {
            rpiCameraHDR: camera.hdr_enabled,
            rpiCameraEV: camera.exposure_compenstion,
            rpiCameraMetering: match camera.metering_mode {
                Metering::Center => "centre".to_owned(),
                Metering::Average => "matrix".to_owned(),
            },
            rpiCameraAfMode: match camera.focus_mode {
                FocusMode::Auto => "continuous".to_owned(),
                FocusMode::Fixed => "manual".to_owned(),
            },
            rpiCameraROI: get_roi(camera),
            rpiCameraBitrate: camera.bitrate,
        }
    }
}
