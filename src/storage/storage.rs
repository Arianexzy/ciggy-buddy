use crate::storage::models::SmokingData;
use chrono::Local;
use parking_lot::Mutex;
use std::fs;
use std::path::PathBuf;

use super::models::SmokingEvent;

static STORAGE: Mutex<Option<SmokingData>> = Mutex::new(None);

pub fn add_smoking_event() {
    let mut storage = STORAGE.lock();
    if storage.is_none() {
        *storage = Some(load_data());
    }

    if let Some(data) = storage.as_mut() {
        data.events.push(SmokingEvent {
            timestamp: Local::now(),
        });
        save_data(data);
    }
}

pub fn load_data() -> SmokingData {
    let path = get_storage_path();

    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap(),
        Err(err) => {
            println!("storage::load_data: Failed to load file: {}", err);
            SmokingData::default()
        }
    }
}

pub fn save_data(data: &SmokingData) {
    let path = get_storage_path();
    if let Ok(json) = serde_json::to_string_pretty(data) {
        fs::write(path, json).expect("storage::save_data: Failed to save data");
    }
}

pub fn get_storage_path() -> PathBuf {
    let path = PathBuf::from("/data/user/0/com.example.CiggyBuddy/files");
    fs::create_dir_all(&path)
        .expect("storage::storage::get_storage_path: Failed to create app storage directory");

    path.join("smoking_data.json")
}

/*
    GETTER FUNCTIONS
*/

pub fn get_total_count() -> i32 {
    get_count(|data| data.total_count())
}

pub fn get_today_count() -> i32 {
    get_count(|data| data.today_count())
}

pub fn get_week_count() -> i32 {
    get_count(|data| data.week_count())
}

pub fn get_month_count() -> i32 {
    get_count(|data| data.month_count())
}

pub fn get_year_count() -> i32 {
    get_count(|data| data.year_count())
}

fn get_count<F>(count_fn: F) -> i32
where
    F: FnOnce(&SmokingData) -> i32,
{
    let mut storage = STORAGE.lock();
    if storage.is_none() {
        *storage = Some(load_data());
    }
    let data = storage.as_ref().unwrap();
    count_fn(data)
}
