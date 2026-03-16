use tauri::State;
use crate::state::AppState;
use crate::db::{models::UrlList, queries};

#[tauri::command]
pub async fn list_url_lists(
    state: State<'_, AppState>,
) -> Result<Vec<UrlList>, String> {
    let db = state.db.lock().await;
    queries::list_url_lists(&db)
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn create_url_list(
    name: String,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().await;
    queries::create_url_list(&db, &name)
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn add_url_to_list(
    list_id: i64,
    url: String,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().await;
    queries::add_url_to_list(&db, list_id, &url)
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn import_url_list(
    list_id: i64,
    file_path: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let db = state.db.lock().await;
    queries::import_url_list_from_file(&db, list_id, &file_path)
        .map_err(|e| format!("Import error: {}", e))
}
