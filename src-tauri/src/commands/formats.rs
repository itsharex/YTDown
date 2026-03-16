use crate::ytdlp::{binary, process};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn fetch_formats(
    url: String,
    state: State<'_, AppState>,
) -> Result<crate::ytdlp::parser::VideoInfo, String> {
    let ytdlp_path = state.ytdlp_path.lock().await;
    let binary = binary::detect_binary(ytdlp_path.as_deref())?;
    process::fetch_info(&binary.path.to_string_lossy(), &url).await
}
