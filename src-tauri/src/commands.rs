use crate::config;
use crate::AppState;
use tauri::{AppHandle, State};
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn get_config(state: State<'_, AppState>) -> config::AppConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
pub fn save_config(
    app: AppHandle,
    state: State<'_, AppState>,
    config: config::AppConfig,
) -> Result<(), String> {
    config::save_config(&config)?;
    let manager = app.autolaunch();
    if config.auto_start {
        manager.enable().map_err(|e| e.to_string())?;
    } else {
        manager.disable().map_err(|e| e.to_string())?;
    }
    *state.config.lock().unwrap() = config;
    Ok(())
}

#[tauri::command]
pub fn get_reader_status(state: State<'_, AppState>) -> bool {
    state
        .reader_connected
        .load(std::sync::atomic::Ordering::SeqCst)
}

#[tauri::command]
pub fn get_queue_size() -> usize {
    crate::api::queue::load_queue().len()
}
