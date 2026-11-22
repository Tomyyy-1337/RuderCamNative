use serde::{Deserialize, Serialize};

use crate::stores::store::Store;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlaySettings {
    show_overlay: bool,
    overlay_position: OverlayPosition,
    elements: [OverlayElement; 6],
}

impl Store for OverlaySettings {
    const KEY: &'static str = "overlay_settings";
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OverlayPosition {
    Top,
    Bottom
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum OverlayElementTypes {
    CurrentSpeed,
    CurrentBPM,
    SessionDistance,
    SessionTime,
    Split500m,
    DistancePerStroke,
    FahrtSchlagCount
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub struct OverlayElement {
    pub element_type: OverlayElementTypes,
    pub visible: bool,
}

impl OverlayElement {
    pub fn new(element_type: OverlayElementTypes, visible: bool) -> Self {
        OverlayElement {
            element_type,
            visible
        }
    }
}