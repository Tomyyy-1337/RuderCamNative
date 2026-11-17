//! The theme mode, kept in the app's storage across launches.

use nsh_core::prelude::Appearance;
use nsh_core::storage::Storage;
use serde::{Deserialize, Serialize};

const KEY: &str = "theme";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn appearance(self) -> Appearance {
        match self {
            Self::Light => Appearance::Light,
            Self::Dark => Appearance::Dark,
        }
    }
}

pub fn load() -> ThemeMode {
    Storage::get_item(KEY)
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

pub fn save(mode: ThemeMode) {
    match serde_json::to_string(&mode) {
        Ok(raw) => {
            if let Err(error) = Storage::set_item(KEY, raw) {
                eprintln!("theme: {error}");
            }
        }
        Err(error) => eprintln!("theme: {error}"),
    }
}
