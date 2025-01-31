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

// fn main() {
//     let file_path = "/data/user/0/com.example.CiggyBuddy/files/smoking_data.json"; // Path to your JSON file
//     let synthetic_events = generate_synthetic_data();
    
//     append_to_json(file_path, synthetic_events);

//     // If needed, clear synthetic data after testing:
//     // fs::write(file_path, "{\"events\": []}").expect("Failed to clear file");

//     println!("Process completed!");
// }
