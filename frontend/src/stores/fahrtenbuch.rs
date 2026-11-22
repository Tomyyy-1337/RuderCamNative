//! The Fahrtenbuch, a log of finished trips kept in the app's storage across launches.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use shared_data::FinishedFahrt;

use crate::stores::store::Store;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fahrtenbuch {
    pub next_index: usize,
    pub entries: VecDeque<Indexed<FinishedFahrt>>,
}

impl Fahrtenbuch {
    pub fn new() -> Self {
        Fahrtenbuch {
            next_index: 0,
            entries: VecDeque::new(),
        }
    }

    pub fn add_entry(&mut self, value: FinishedFahrt) {
        let entry = Indexed {
            index: self.next_index,
            value,
        };
        self.entries.push_back(entry);
        self.next_index += 1;
    }

    pub fn remove_entry(&mut self, index: usize) {
        self.entries.retain(|entry| entry.index != index);
    }

    pub fn total_distance(&self) -> f32 {
        self.entries.iter().map(|entry| entry.value.distance_traveled).sum()
    }

    pub fn newest_first_ids(&self) -> Vec<usize> {
        self.entries.iter().rev().map(|entry| entry.index).collect()
    }

    pub fn find_trip(&self, id: usize) -> Option<&FinishedFahrt> {
        self.entries.iter().find(|entry| entry.index == id).map(|entry| &entry.value)
    }
}

impl Store for Fahrtenbuch {
    const KEY: &'static str = "fahrtenbuch";
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Indexed<T> {
    pub index: usize,
    pub value: T,
}
