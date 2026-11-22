//! The theme mode, kept in the app's storage across launches.

use nsh_core::prelude::Appearance;
use nsh_core::storage::Storage;
use serde::{Deserialize, Serialize};

use crate::stores::store::Store;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl Store for ThemeMode {
    const KEY: &'static str = "theme";
}

impl ThemeMode {
    pub fn appearance(self) -> Appearance {
        match self {
            Self::Light => Appearance::Light,
            Self::Dark => Appearance::Dark,
        }
    }
}
