#[tauri::command]
pub async fn import_cookies_from_browser(browser: String) -> Result<String, String> {
    // Validate browser name
    match browser.as_str() {
        "safari" | "chrome" | "firefox" => Ok(browser),
        _ => Err(format!("Unsupported browser: {}", browser)),
    }
    // NOTE: Actual cookie import is handled by passing --cookies-from-browser to yt-dlp
    // This command just validates and stores the preference
}

#[tauri::command]
pub async fn set_cookie_file(path: String) -> Result<(), String> {
    if !path.is_empty() && !std::path::Path::new(&path).exists() {
        return Err(format!("Cookie file not found: {}", path));
    }
    Ok(())
}
