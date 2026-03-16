use std::path::PathBuf;
use std::process::Command;

#[derive(Debug)]
pub enum ManagedBy {
    Homebrew,
    Bundled,
    Manual,
}

pub struct YtdlpBinary {
    pub path: PathBuf,
    pub version: String,
    pub managed_by: ManagedBy,
}

/// Detect yt-dlp binary following priority: manual path > system PATH > bundled
pub fn detect_binary(manual_path: Option<&str>) -> Result<YtdlpBinary, String> {
    // 1. Manual path
    if let Some(path) = manual_path {
        if path != "auto" {
            let pb = PathBuf::from(path);
            if pb.exists() {
                let version = get_version(&pb)?;
                return Ok(YtdlpBinary {
                    path: pb,
                    version,
                    managed_by: ManagedBy::Manual,
                });
            }
            return Err(format!("Manual yt-dlp path not found: {}", path));
        }
    }

    // 2. System PATH (using `which` crate for cross-platform support)
    if let Ok(path) = which::which("yt-dlp") {
        if path.exists() {
            let path_str = path.to_string_lossy().to_string();
            let pb = path;
            let version = get_version(&pb)?;
            let managed_by = if path_str.contains("Cellar") || path_str.contains("homebrew") {
                ManagedBy::Homebrew
            } else {
                ManagedBy::Manual
            };
            return Ok(YtdlpBinary {
                path: pb,
                version,
                managed_by,
            });
        }
    }

    // 3. Bundled
    let bundled = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join("YTDown")
        .join("bin")
        .join("yt-dlp");
    if bundled.exists() {
        let version = get_version(&bundled)?;
        return Ok(YtdlpBinary {
            path: bundled,
            version,
            managed_by: ManagedBy::Bundled,
        });
    }

    Err("yt-dlp not found. Install via homebrew: brew install yt-dlp".to_string())
}

fn get_version(path: &PathBuf) -> Result<String, String> {
    let output = Command::new(path)
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to run yt-dlp: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Download yt-dlp binary to app's bundled location
pub fn download_ytdlp_binary() -> Result<PathBuf, String> {
    let target_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join("YTDown")
        .join("bin");
    std::fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Failed to create bin dir: {}", e))?;
    let target_path = target_dir.join("yt-dlp");

    let url = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos";
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Download failed: {}", e))?;
    let bytes = response.bytes()
        .map_err(|e| format!("Failed to read response: {}", e))?;

    std::fs::write(&target_path, &bytes)
        .map_err(|e| format!("Failed to write binary: {}", e))?;

    // Make executable
    Command::new("chmod")
        .args(["+x", &target_path.to_string_lossy()])
        .output()
        .map_err(|e| format!("Failed to chmod: {}", e))?;

    Ok(target_path)
}

/// Check if homebrew has updates for yt-dlp
pub fn check_homebrew_update() -> Result<bool, String> {
    let output = Command::new("brew")
        .args(["outdated", "yt-dlp"])
        .output()
        .map_err(|e| format!("Failed to check brew: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(!stdout.trim().is_empty())
}
