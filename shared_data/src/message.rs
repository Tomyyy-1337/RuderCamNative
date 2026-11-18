use serde::{Deserialize, Serialize};

use crate::{BatteryState, CameraInterface};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Message {
    CameraInterfaceUpdate(CameraInterface),
    BatteryState(BatteryState),
    GpsSateliteCount(u8),
}