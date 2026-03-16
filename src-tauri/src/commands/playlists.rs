use tauri::State;
use crate::state::AppState;
use crate::db::{models::{Playlist, PlaylistItem}, queries};

#[tauri::command]
pub async fn list_playlists(
    state: State<'_, AppState>,
) -> Result<Vec<Playlist>, String> {
    let db = state.db.lock().await;
    queries::list_playlists(&db)
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn create_playlist(
    name: String,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().await;
    queries::create_playlist(&db, &name)
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn delete_playlist(
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().await;
    queries::delete_playlist(&db, id)
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn add_playlist_item(
    playlist_id: i64,
    download_id: Option<i64>,
    url: Option<String>,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let db = state.db.lock().await;
    queries::add_playlist_item(&db, playlist_id, download_id, url.as_deref())
        .map_err(|e| format!("DB error: {}", e))
}

#[tauri::command]
pub async fn get_playlist_items(
    playlist_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<PlaylistItem>, String> {
    let db = state.db.lock().await;
    queries::get_playlist_items(&db, playlist_id)
        .map_err(|e| format!("DB error: {}", e))
}
