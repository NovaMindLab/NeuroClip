pub mod ffmpeg;
pub mod highlight;
pub mod tts;
pub mod script;
pub mod commands;
pub mod automation;
pub mod db;
pub mod scanner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_system_status,
            commands::start_auto_clip,
            commands::toggle_folder_watcher,
            commands::get_system_scan_dirs,
            commands::start_video_scan,
            commands::cancel_video_scan,
            commands::query_video_assets,
            commands::get_library_metrics,
            commands::probe_video_asset,
            commands::delete_video_asset,
        ])
        .run(tauri::generate_context!())
        .expect("运行 NeuroClip Tauri 应用发生错误");
}
