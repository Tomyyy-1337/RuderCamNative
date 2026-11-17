//! The Fahrtenbuch, a log of finished trips kept in the app's storage across launches.

use std::collections::VecDeque;
use std::time::Duration;

use chrono::{DateTime, Datelike, Local, Timelike};
use nsh_core::storage::Storage;
use serde::{Deserialize, Serialize};

const KEY: &str = "fahrtenbuch";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SerializableDateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl SerializableDateTime {
    pub fn date_string(&self) -> String {
        format!("{:02}.{:02}.{:04}", self.day, self.month, self.year)
    }

    pub fn hour_minute_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    pub fn year_month(&self) -> (u16, u8) {
        (self.year, self.month)
    }

    pub fn month_label(&self) -> String {
        const MONTHS: [&str; 12] = [
            "Januar", "Februar", "März", "April", "Mai", "Juni",
            "Juli", "August", "September", "Oktober", "November", "Dezember",
        ];
        let name = MONTHS.get(self.month.wrapping_sub(1) as usize).copied().unwrap_or("?");
        format!("{} {}", name, self.year)
    }
}

impl From<DateTime<Local>> for SerializableDateTime {
    fn from(dt: DateTime<Local>) -> Self {
        SerializableDateTime {
            year: dt.year() as u16,
            month: dt.month() as u8,
            day: dt.day() as u8,
            hour: dt.hour() as u8,
            minute: dt.minute() as u8,
            second: dt.second() as u8,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct FinishedFahrt {
    pub client_start_time: SerializableDateTime,
    pub schlag_count: u32,
    pub distance_traveled: f32,
    pub gps_history: Vec<GpsPosition>,
    pub active_time: std::time::Duration,
}

impl FinishedFahrt {
    pub fn active_time_string(&self) -> String {
        let total = self.active_time.as_secs();
        format!("{:02}:{:02}", total / 3600, (total / 60) % 60)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Indexed<T> {
    pub index: usize,
    pub value: T,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct GpsPosition {
    pub latitude: f64,
    pub longitude: f64,
}

/// The stored Fahrtenbuch, or the sample trips when none is stored or it is unreadable.
pub fn load() -> Fahrtenbuch {
    Storage::get_item(KEY)
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_else(sample)
}

pub fn save(book: &Fahrtenbuch) {
    match serde_json::to_string(book) {
        Ok(raw) => {
            if let Err(error) = Storage::set_item(KEY, raw) {
                eprintln!("fahrtenbuch: {error}");
            }
        }
        Err(error) => eprintln!("fahrtenbuch: {error}"),
    }
}

/// Fake trips for the first launch, before any trip is stored.
pub fn sample() -> Fahrtenbuch {
    let mut book = Fahrtenbuch::new();
    for i in 0..90u32 {
        book.add_entry(fake_trip(
            5 + (i / 15) as u8,
            ((i % 15) * 2 + 1) as u8,
            (6 + i % 12) as u8,
            ((i * 7) % 60) as u8,
            3.0 + (i % 17) as f32 * 1.5,
            1 + i % 6,
            600 + u64::from(i) * 45,
            (52.5 + f64::from(i) * 0.001, 13.4),
        ));
    }
    book.add_entry(fake_trip(11, 10, 8, 15, 12.4, 3, 2700, (52.5200, 13.4050)));
    book.add_entry(fake_trip(11, 12, 13, 40, 18.9, 5, 3900, (52.5124, 13.4220)));
    book.add_entry(fake_trip(11, 13, 7, 5, 7.6, 2, 1320, (52.5095, 13.4480)));
    book.add_entry(fake_trip(11, 14, 16, 22, 25.3, 8, 5410, (52.4930, 13.4650)));
    book
}

fn fake_trip(
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    distance_traveled: f32,
    schlag_count: u32,
    active_secs: u64,
    start: (f64, f64),
) -> FinishedFahrt {
    let gps_history = (0..8u8)
        .map(|i| GpsPosition {
            latitude: start.0 + 0.004 * f64::from(i),
            longitude: start.1 + 0.006 * f64::from(i),
        })
        .collect();
    FinishedFahrt {
        client_start_time: SerializableDateTime {
            year: 2026,
            month,
            day,
            hour,
            minute,
            second: 0,
        },
        schlag_count,
        distance_traveled,
        gps_history,
        active_time: Duration::from_secs(active_secs),
    }
}
