use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct BatteryState {
    pub percentage: u8,
}