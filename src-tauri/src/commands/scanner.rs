use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tauri::{AppHandle, Emitter};
use serde::{Deserialize, Serialize};

use crate::db::{DatabaseManager, LibraryStats};
use crate::scanner::{VideoScanEngine, LightProbeEngine};
use crate::ffmpeg::FFmpegContext;

static DB_INSTANCE: OnceLock<DatabaseManager> = OnceLock::new();
static SCANNER_INSTANCE: OnceLock<Arc<VideoScanEngine>> = OnceLock::new();

pub fn get_db() -> &'static DatabaseManager {
    DB_INSTANCE.get_or_init(|| {
        let base_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".neuroclip");
        let db_path = base_dir.join("neuroclip_assets.db");
        DatabaseManager::new(&db_path).expect("初始化 SQLite 数据库失败")
    })
}

pub fn get_scanner() -> &'static Arc<VideoScanEngine> {
    SCANNER_INSTANCE.get_or_init(|| {
        Arc::new(VideoScanEngine::new(get_db().clone()))
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgressEvent {
    pub current_directory: String,
    pub batch_count: usize,
    pub total_discovered: usize,
    pub is_finished: bool,
}

#[tauri::command]
pub fn get_system_scan_dirs() -> Vec<String> {
    let mut dirs_list = Vec::new();

    if let Some(video_dir) = dirs::video_dir() {
        if video_dir.exists() {
            dirs_list.push(video_dir.to_string_lossy().to_string());
        }
    }
    if let Some(download_dir) = dirs::download_dir() {
        if download_dir.exists() {
            dirs_list.push(download_dir.to_string_lossy().to_string());
        }
    }
    if let Some(desktop_dir) = dirs::desktop_dir() {
        if desktop_dir.exists() {
            dirs_list.push(desktop_dir.to_string_lossy().to_string());
        }
    }

    dirs_list
}

#[tauri::command]
pub async fn start_video_scan(
    app: AppHandle,
    custom_dirs: Option<Vec<String>>,
) -> Result<String, String> {
    let target_paths: Vec<PathBuf> = match custom_dirs {
        Some(custom) if !custom.is_empty() => custom.into_iter().map(PathBuf::from).collect(),
        _ => get_system_scan_dirs().into_iter().map(PathBuf::from).collect(),
    };

    if target_paths.is_empty() {
        return Err("未指定有效的扫描目录".to_string());
    }

    let scanner = get_scanner().clone();
    let app_clone = app.clone();

    tokio::task::spawn_blocking(move || {
        let res = scanner.scan_directories(target_paths, {
            let app_inner = app_clone.clone();
            move |batch_cnt, total_cnt, current_dir| {
                let _ = app_inner.emit(
                    "scan://progress",
                    ScanProgressEvent {
                        current_directory: current_dir.to_string(),
                        batch_count: batch_cnt,
                        total_discovered: total_cnt,
                        is_finished: false,
                    },
                );
            }
        });

        let total = res.unwrap_or(0);
        let _ = app_clone.emit(
            "scan://progress",
            ScanProgressEvent {
                current_directory: "完成".to_string(),
                batch_count: 0,
                total_discovered: total,
                is_finished: true,
            },
        );
    });

    Ok("扫描作业已在后台启动".to_string())
}

#[tauri::command]
pub fn cancel_video_scan() -> Result<(), String> {
    get_scanner().cancel();
    Ok(())
}

#[tauri::command]
pub fn query_video_assets(
    page: u32,
    page_size: u32,
    search: Option<String>,
    format_filter: Option<String>,
    sort_by: Option<String>,
) -> Result<serde_json::Value, String> {
    let (total, items) = get_db()
        .query_assets(
            page,
            page_size,
            search.as_deref(),
            format_filter.as_deref(),
            sort_by.as_deref(),
        )?;

    Ok(serde_json::json!({
        "total": total,
        "page": page,
        "page_size": page_size,
        "items": items
    }))
}

#[tauri::command]
pub fn get_library_metrics() -> Result<LibraryStats, String> {
    get_db().get_library_stats()
}

#[tauri::command]
pub async fn probe_video_asset(
    asset_id: i64,
    file_path: String,
) -> Result<serde_json::Value, String> {
    let ctx = FFmpegContext::discover().map_err(|e| e)?;
    let p = PathBuf::from(&file_path);
    if !p.exists() {
        return Err("物理文件不存在".to_string());
    }

    let probe_res = LightProbeEngine::probe_video(&ctx.ffprobe_path, &p)?;
    get_db().update_probe_metadata(asset_id, probe_res.duration, probe_res.width, probe_res.height)?;

    Ok(serde_json::json!({
        "id": asset_id,
        "duration": probe_res.duration,
        "width": probe_res.width,
        "height": probe_res.height,
        "status": "ready"
    }))
}

#[tauri::command]
pub fn delete_video_asset(asset_id: i64) -> Result<(), String> {
    get_db().delete_asset(asset_id)
}
