use crate::storage::time_utils;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SmokingEvent {
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SmokingData {
    pub events: Vec<SmokingEvent>,
}

impl SmokingData {
    pub fn total_count(&self) -> i32 {
        self.events.len() as i32
    }

    pub fn today_count(&self) -> i32 {
        let now = Local::now();
        self.events
            .iter()
            .filter(|event| time_utils::is_same_day(&event.timestamp, &now))
            .count() as i32
    }

    pub fn week_count(&self) -> i32 {
        let now = Local::now();
        self.events
            .iter()
            .filter(|event| time_utils::is_same_week(&event.timestamp, &now))
            .count() as i32
    }

    pub fn month_count(&self) -> i32 {
        let now = Local::now();
        self.events
            .iter()
            .filter(|event| time_utils::is_same_month(&event.timestamp, &now))
            .count() as i32
    }

    pub fn year_count(&self) -> i32 {
        let now = Local::now();
        self.events
            .iter()
            .filter(|event| time_utils::is_same_year(&event.timestamp, &now))
            .count() as i32
    }
}
