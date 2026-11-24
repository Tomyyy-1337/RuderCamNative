use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WlanSettings {
    pub ssid: String,
    pub password: String,
}

impl Default for WlanSettings {
    fn default() -> Self {
        WlanSettings {
            ssid: "RuderCam".to_string(),
            password: "ObilanKenobi".to_string()
        }
    }
}