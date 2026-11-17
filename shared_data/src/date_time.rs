use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SerializableDateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[cfg(feature = "frontend")]
use chrono::{DateTime, Datelike, Local, Timelike};

#[cfg(feature = "frontend")]
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

#[cfg(feature = "frontend")]
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