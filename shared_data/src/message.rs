use serde::{Deserialize, Serialize};

use crate::{BatteryState, CameraInterface, CurrentStats, FahrtInfo, FinishedFahrt};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Message {
    CameraInterfaceUpdate(CameraInterface),
    BatteryState(BatteryState),
    GpsSateliteCount(u8),
    FinishedFahrt(FinishedFahrt),
    CurrentStats(CurrentStats),
    FahrtInfoUpdate(Option<FahrtInfo>)
}