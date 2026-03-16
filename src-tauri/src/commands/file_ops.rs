use std::fs;
use std::process::Command;

#[tauri::command]
pub async fn move_file(source: String, destination: String) -> Result<(), String> {
    fs::rename(&source, &destination)
        .map_err(|e| format!("Failed to move file: {}", e))
}

#[tauri::command]
pub async fn delete_file(path: String, to_trash: bool) -> Result<(), String> {
    if to_trash {
        // macOS: use osascript to move to trash
        Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "tell application \"Finder\" to delete POSIX file \"{}\"",
                    path
                ),
            ])
            .output()
            .map_err(|e| format!("Failed to trash file: {}", e))?;
        Ok(())
    } else {
        fs::remove_file(&path).map_err(|e| format!("Failed to delete file: {}", e))
    }
}

#[tauri::command]
pub async fn reveal_in_finder(path: String) -> Result<(), String> {
    Command::new("open")
        .args(["-R", &path])
        .spawn()
        .map_err(|e| format!("Failed to reveal in Finder: {}", e))?;
    Ok(())
}
