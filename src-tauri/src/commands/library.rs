use tauri::State;
use crate::state::AppState;
use crate::db::{models::Download, queries};

#[tauri::command]
pub async fn list_library(
    status_filter: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Download>, String> {
    let db = state.db.lock().await;
    queries::list_downloads(&db, status_filter.as_deref())
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn search_library(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<Download>, String> {
    let db = state.db.lock().await;
    queries::search_downloads(&db, &query)
        .map_err(|e| format!("Search error: {}", e))
}

#[tauri::command]
pub async fn toggle_favorite(
    id: i64,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let db = state.db.lock().await;
    queries::toggle_favorite(&db, id)
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn get_download(
    id: i64,
    state: State<'_, AppState>,
) -> Result<Download, String> {
    let db = state.db.lock().await;
    queries::get_download(&db, id)
        .map_err(|e| format!("DB error: {}", e))
}
