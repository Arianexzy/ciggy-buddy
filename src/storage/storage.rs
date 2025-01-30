use crate::storage::models::SmokingData;
use chrono::Local;
use directories::ProjectDirs;
use parking_lot::Mutex;
use std::fs;
use std::path::PathBuf;

use super::models::SmokingEvent;

static STORAGE: Mutex<Option<SmokingData>> = Mutex::new(None);

pub fn get_storage_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "example", "CiggyBuddy")
        .expect("storage::storage::get_storage_path: Failed to get project directories");

    let storage_dir = proj_dirs.data_dir();
    fs::create_dir_all(storage_dir)
        .expect("storage::storage::get_storage_path: Failed to create storage directory");

    storage_dir.join("smoking_data.json")
}

pub fn load_data() -> SmokingData {
    let path = get_storage_path();

    fs::read_to_string(&path)
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default()
}

pub fn save_data(data: &SmokingData) {
    let path = get_storage_path();
    if let Ok(json) = serde_json::to_string_pretty(data) {
        fs::write(path, json).expect("storage::save_data: Failed to save data");
    }
}

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

pub fn get_total_count() -> i32 {
    let storage = STORAGE.lock();
    let binding = SmokingData::default();
    let data = storage.as_ref().unwrap_or(&binding);
    
    data.total_count()
}