use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct GpsPosition {
    pub latitude: f64,
    pub longitude: f64,
}