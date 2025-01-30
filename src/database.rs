use std::path::PathBuf;

fn init_db() -> rusqlite::Connection {
    let db_path = get_db_path();
    
    let connection = rusqlite::Connection::open(&db_path)
        .expect("Failed to open database");
    
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS butt_buddy (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            count INTEGER NOT NULL DEFAULT 1,
            last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );"
    ).unwrap();
        
    connection
}

fn get_db_path() -> PathBuf {
    let project_dirs = directories::ProjectDirs::from("com", "example", "CiggyBuddy")
        .expect("Failed to get project directories");
    
    let data_dir = project_dirs.data_dir();
    
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)
            .expect("Failed create a data directory");
    }
    
    data_dir.join("ciggybuddy.db")
}

pub async fn save_total_count() {
    init_db()
        .execute("INSERT INTO ciggy_buddy (count) VALUES (1);", [])
        .expect("save_total_count(): Failed to insert new count");
}