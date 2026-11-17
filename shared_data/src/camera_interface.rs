use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraInterface {
    pub hdr_enabled: bool,
    pub exposure_compenstion: f32,
    pub metering_mode: Metering,
    pub focus_mode: FocusMode,
    pub focal_length: u8,
    pub bitrate: u32,
}

impl CameraInterface {
    pub fn new() -> Self {
        CameraInterface {
            hdr_enabled: true,
            exposure_compenstion: 0.0,
            metering_mode: Metering::Average,
            focus_mode: FocusMode::Fixed,
            focal_length: 28,
            bitrate: 3_200_000,
        }
    }
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum Metering {
    Center,
    Average,
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Deserialize)]
pub enum FocusMode {
    Auto,
    Fixed,
}
