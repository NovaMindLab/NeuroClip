pub mod ffmpeg;
pub mod highlight;
pub mod tts;
pub mod script;
pub mod commands;
pub mod automation;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_system_status,
            commands::start_auto_clip,
            commands::toggle_folder_watcher,
        ])
        .run(tauri::generate_context!())
        .expect("运行 NeuroClip Tauri 应用发生错误");
}
