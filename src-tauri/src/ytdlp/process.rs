use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tauri::{AppHandle, Emitter};

use super::parser::{parse_progress_line, parse_video_info, VideoInfo};

pub struct DownloadConfig {
    pub ytdlp_path: String,
    pub url: String,
    pub format: String,
    pub quality: String,
    pub output_dir: String,
    pub output_template: String,
    pub embed_thumbnail: bool,
    pub embed_metadata: bool,
    pub write_subs: bool,
    pub embed_subs: bool,
    pub embed_chapters: bool,
    pub sponsorblock: bool,
    pub custom_format: Option<String>,
    pub cookie_browser: Option<String>,
    pub cookie_file: Option<String>,
}

/// Fetch video info (formats, metadata) without downloading
pub async fn fetch_info(ytdlp_path: &str, url: &str) -> Result<VideoInfo, String> {
    let output = Command::new(ytdlp_path)
        .args(["--dump-json", "--no-download", url])
        .output()
        .await
        .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("yt-dlp error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_video_info(&stdout)
}

/// Start a download, emitting progress events
pub async fn start_download(
    app: AppHandle,
    download_id: i64,
    config: DownloadConfig,
) -> Result<u32, String> {
    let mut args = vec![
        "--newline".to_string(),
        "--progress".to_string(),
    ];

    // Format selection
    if let Some(ref custom) = config.custom_format {
        args.extend(["-f".to_string(), custom.clone()]);
    } else {
        let format_str = build_format_string(&config.format, &config.quality);
        args.extend(["-f".to_string(), format_str]);
    }

    // Output
    let output_path = format!("{}/{}", config.output_dir, config.output_template);
    args.extend(["-o".to_string(), output_path]);

    // Post-process options
    if config.embed_thumbnail { args.push("--embed-thumbnail".to_string()); }
    if config.embed_metadata { args.push("--embed-metadata".to_string()); }
    if config.write_subs { args.extend(["--write-subs".to_string(), "--write-auto-subs".to_string()]); }
    if config.embed_subs { args.push("--embed-subs".to_string()); }
    if config.embed_chapters { args.push("--embed-chapters".to_string()); }
    if config.sponsorblock { args.push("--sponsorblock-remove".to_string()); }

    // Cookies
    if let Some(ref browser) = config.cookie_browser {
        if browser != "none" {
            args.extend(["--cookies-from-browser".to_string(), browser.clone()]);
        }
    }
    if let Some(ref cookie_file) = config.cookie_file {
        if !cookie_file.is_empty() {
            args.extend(["--cookies".to_string(), cookie_file.clone()]);
        }
    }

    args.push(config.url);

    let mut child = Command::new(&config.ytdlp_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn yt-dlp: {}", e))?;

    let pid = child.id().ok_or("Failed to get PID")?;

    // Stream stdout for progress
    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let app_clone = app.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            if let Some(progress) = parse_progress_line(&line) {
                let event = serde_json::json!({
                    "download_id": download_id,
                    "percent": progress.percent,
                    "speed_bps": progress.speed_bps,
                    "speed_str": progress.speed_str,
                    "eta_secs": progress.eta_secs,
                    "eta_str": progress.eta_str,
                    "downloaded_bytes": progress.downloaded_bytes,
                    "total_bytes": progress.total_bytes,
                    "status": "downloading",
                });
                let _ = app_clone.emit("download-progress", event);
            }
        }
    });

    Ok(pid)
}

fn build_format_string(format: &str, quality: &str) -> String {
    match format {
        "mp3" | "m4a" | "flac" | "wav" | "opus" => {
            "bestaudio/best".to_string()
        }
        _ => {
            let height = match quality {
                "4k" | "2160" => "2160",
                "1080" => "1080",
                "720" => "720",
                "480" => "480",
                _ => return "bestvideo+bestaudio/best".to_string(),
            };
            format!("bestvideo[height<={}]+bestaudio/best", height)
        }
    }
}
