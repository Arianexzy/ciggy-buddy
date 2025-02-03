use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{Utc, Duration};
use rand::Rng;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmokingData {
    pub events: Vec<Event>,
}

// Generate a random timestamp with an offset in days
pub fn generate_timestamp(offset: i64) -> String {
    let now = Utc::now();
    let random_time = now + Duration::minutes(rand::thread_rng().gen_range(1..60));
    let final_time = random_time + Duration::days(offset);
    final_time.to_rfc3339()
}

// Generate synthetic smoking events
pub fn generate_synthetic_data() -> Vec<Event> {
    let mut data = Vec::new();

    // Add synthetic entries for today
    for _ in 0..10 {
        let timestamp = generate_timestamp(0);
        data.push(Event { timestamp });
    }

    // Add synthetic entries for the past week
    for i in 1..=7 {
        let timestamp = generate_timestamp(-i);
        data.push(Event { timestamp });
    }

    // Add synthetic entries for the past month
    for i in 1..=30 {
        let timestamp = generate_timestamp(-i);
        data.push(Event { timestamp });
    }

    // Add synthetic entries for the past year
    for i in 1..=365 {
        let timestamp = generate_timestamp(-i);
        data.push(Event { timestamp });
    }

    data
}

// Append generated events to the JSON file
pub fn append_to_json(file_path: &str, synthetic_events: Vec<Event>) {
    // Read the existing JSON file
    let file_content = if let Ok(content) = fs::read_to_string(file_path) {
        content
    } else {
        String::from("{\"events\": []}") // If the file doesn't exist, start with an empty events list
    };

    // Parse existing JSON data
    let mut smoking_data: SmokingData = serde_json::from_str(&file_content).unwrap_or(SmokingData { events: Vec::new() });

    // Append new synthetic data
    smoking_data.events.extend(synthetic_events);

    // Write back to the file
    let new_content = serde_json::to_string_pretty(&smoking_data).unwrap();
    fs::write(file_path, new_content).expect("Failed to write to file");

    println!("Synthetic data successfully added!");
}

// Generate a random timestamp within January 2025
pub fn generate_january_2025_timestamp() -> String {
    let mut rng = rand::rng();
    
    // Generate a random day in January (1-31)
    let day = rng.gen_range(1..=31);
    
    // Generate random hour and minute
    let hour = rng.gen_range(0..24);
    let minute = rng.gen_range(0..60);
    
    // Create timestamp for January 2025
    let naive_datetime = NaiveDateTime::parse_from_str(
        &format!("2025-01-{:02} {:02}:{:02}:00", day, hour, minute),
        "%Y-%m-%d %H:%M:%S"
    ).unwrap();
    
    let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);
    datetime_utc.to_rfc3339()
}

// Generate exactly 100 smoking events for January 2025
pub fn generate_january_2025_data() -> Vec<Event> {
    let mut data = Vec::new();

    // Generate 100 events
    for _ in 0..100 {
        let timestamp = generate_january_2025_timestamp();
        data.push(Event { timestamp });
    }

    // Sort the events by timestamp
    data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    data
}

// Write the events to the JSON file (overwrites existing data)
pub fn write_to_json(file_path: &str, events: Vec<Event>) {
    let smoking_data = SmokingData { events };
    let content = serde_json::to_string_pretty(&smoking_data).unwrap();
    fs::write(file_path, content).expect("Failed to write to file");
    println!("January 2025 data successfully written!");
}

pub fn generate_test_data(file_path: &str) {
    let january_events = generate_january_2025_data();
    write_to_json(file_path, january_events);
    println!("Test data generation completed!");
}

// fn main() {
//     let file_path = "/data/user/0/com.example.CiggyBuddy/files/smoking_data.json"; // Path to your JSON file
//     let synthetic_events = generate_synthetic_data();
    
//     append_to_json(file_path, synthetic_events);

//     // If needed, clear synthetic data after testing:
//     // fs::write(file_path, "{\"events\": []}").expect("Failed to clear file");

//     println!("Process completed!");
// }
