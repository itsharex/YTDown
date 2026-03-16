pub mod models;
pub mod queries;

use rusqlite::Connection;
use std::path::PathBuf;

/// Initialize database: create file if needed, run schema
pub fn init_db(app_data_dir: &PathBuf) -> Result<Connection, String> {
    std::fs::create_dir_all(app_data_dir)
        .map_err(|e| format!("Failed to create data dir: {}", e))?;
    let db_path = app_data_dir.join("ytdown.db");
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open DB: {}", e))?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .map_err(|e| format!("Failed to set pragmas: {}", e))?;
    conn.execute_batch(include_str!("schema.sql"))
        .map_err(|e| format!("Failed to run schema: {}", e))?;
    Ok(conn)
}
