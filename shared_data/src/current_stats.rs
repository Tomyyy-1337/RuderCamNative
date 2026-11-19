use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CurrentStats {
    pub bpm: f32,
    pub velocity: f32,
}