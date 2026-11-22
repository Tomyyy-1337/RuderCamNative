use serde::{Deserialize, Serialize};

use crate::stores::store::Store;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlaySettings {
    show_overlay: bool,
    overlay_position: OverlayPosition,
    elements: Vec<OverlayElement>,
}

impl Default for OverlaySettings {
    fn default() -> Self {
        Self {
            show_overlay: true,
            overlay_position: OverlayPosition::Top,
            elements: OverlayElementTypes::ALL
                .iter()
                .map(|&element_type| OverlayElement::new(element_type, true))
                .collect(),
        }
    }
}

impl Store for OverlaySettings {
    const KEY: &'static str = "overlay_settings";
}

impl OverlaySettings {
    pub fn show_overlay(&self) -> bool {
        self.show_overlay
    }

    pub fn set_show_overlay(&mut self, show: bool) {
        self.show_overlay = show;
    }

    pub fn overlay_position(&self) -> OverlayPosition {
        self.overlay_position
    }

    pub fn set_overlay_position(&mut self, position: OverlayPosition) {
        self.overlay_position = position;
    }

    pub fn elements(&self) -> &[OverlayElement] {
        &self.elements
    }

    /// Moves the element at `from` to `to`. Out-of-range indices are ignored.
    pub fn move_element(&mut self, from: usize, to: usize) {
        if from >= self.elements.len() || to >= self.elements.len() {
            return;
        }
        let element = self.elements.remove(from);
        self.elements.insert(to, element);
    }

    pub fn set_element_visibility(&mut self, index: usize, visible: bool) {
        if let Some(element) = self.elements.get_mut(index) {
            element.visible = visible;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OverlayPosition {
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum OverlayElementTypes {
    CurrentSpeed,
    CurrentBPM,
    SessionDistance,
    SessionTime,
    Split500m,
    DistancePerStroke,
    FahrtSchlagCount,
}

impl OverlayElementTypes {
    pub const ALL: [OverlayElementTypes; 7] = [
        OverlayElementTypes::CurrentSpeed,
        OverlayElementTypes::CurrentBPM,
        OverlayElementTypes::SessionDistance,
        OverlayElementTypes::SessionTime,
        OverlayElementTypes::Split500m,
        OverlayElementTypes::DistancePerStroke,
        OverlayElementTypes::FahrtSchlagCount,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::CurrentSpeed => "Aktuelle Geschwindigkeit",
            Self::CurrentBPM => "Aktuelle Schlagfrequenz",
            Self::SessionDistance => "Strecke",
            Self::SessionTime => "Zeit",
            Self::Split500m => "Split 500 m",
            Self::DistancePerStroke => "Distanz pro Schlag",
            Self::FahrtSchlagCount => "Schlaganzahl",
        }
    }
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
