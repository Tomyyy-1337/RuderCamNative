use serde::{Deserialize, Serialize};

use crate::{GpsPosition, SerializableDateTime};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FinishedFahrt {
    pub client_start_time: SerializableDateTime,
    pub schlag_count: u32,
    pub distance_traveled: f32,
    pub gps_history: Vec<GpsPosition>,
    pub active_time: std::time::Duration,
}

#[cfg(feature = "frontend")]
impl FinishedFahrt {
    pub fn active_time_string(&self) -> String {
        let total = self.active_time.as_secs();
        format!("{:02}:{:02}", total / 3600, (total / 60) % 60)
    }
}