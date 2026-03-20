use std::fs;
use std::process::Command;
use tauri::State;
use crate::state::AppState;

#[tauri::command]
pub async fn move_file(
    source: String,
    destination: String,
    download_id: Option<i64>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Create destination directory if needed
    if let Some(parent) = std::path::Path::new(&destination).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create destination dir: {}", e))?;
    }

    // fs::rename fails across different volumes; fall back to copy + delete
    if let Err(_) = fs::rename(&source, &destination) {
        fs::copy(&source, &destination)
            .map_err(|e| format!("Failed to copy file: {}", e))?;
        fs::remove_file(&source)
            .map_err(|e| format!("Copied but failed to remove original: {}", e))?;
    }

    // Update DB file_path if download_id provided
    if let Some(id) = download_id {
        let db = state.db.lock().await;
        let _ = db.execute(
            "UPDATE downloads SET file_path = ?1 WHERE id = ?2",
            rusqlite::params![destination, id],
        );
    }

    Ok(())
}

/// Move file to trash (platform-specific)
fn move_to_trash(path: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
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
    }
    #[cfg(target_os = "windows")]
    {
        // Use PowerShell to move to recycle bin
        let ps_script = format!(
            "Add-Type -AssemblyName Microsoft.VisualBasic; [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteFile('{}', 'OnlyErrorDialogs', 'SendToRecycleBin')",
            path.replace("'", "''")
        );
        Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_script])
            .output()
            .map_err(|e| format!("Failed to trash file: {}", e))?;
        Ok(())
    }
    #[cfg(target_os = "linux")]
    {
        // Try gio trash first, fall back to gvfs-trash
        let result = Command::new("gio")
            .args(["trash", path])
            .output();
        match result {
            Ok(output) if output.status.success() => Ok(()),
            _ => {
                Command::new("gvfs-trash")
                    .arg(path)
                    .output()
                    .map_err(|e| format!("Failed to trash file: {}", e))?;
                Ok(())
            }
        }
    }
}

#[tauri::command]
pub async fn delete_file(
    path: Option<String>,
    to_trash: bool,
    download_id: Option<i64>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Delete the physical file if path is provided
    if let Some(ref path) = path {
        if std::path::Path::new(path).exists() {
            if to_trash {
                move_to_trash(path)?;
            } else {
                fs::remove_file(path).map_err(|e| format!("Failed to delete file: {}", e))?;
            }
        }
    }

    // Delete the DB record
    if let Some(id) = download_id {
        let db = state.db.lock().await;
        db.execute(
            "DELETE FROM downloads WHERE id = ?1",
            rusqlite::params![id],
        ).map_err(|e| format!("Failed to delete DB record: {}", e))?;
    }

    Ok(())
}

/// Reveal file in the native file manager
#[tauri::command]
pub async fn reveal_in_finder(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| format!("Failed to reveal in Finder: {}", e))?;
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args([&format!("/select,{}", path)])
            .spawn()
            .map_err(|e| format!("Failed to reveal in Explorer: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        // Open the parent directory
        let parent = std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());
        Command::new("xdg-open")
            .arg(&parent)
            .spawn()
            .map_err(|e| format!("Failed to open file manager: {}", e))?;
    }
    Ok(())
}
