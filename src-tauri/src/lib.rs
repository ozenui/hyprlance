mod platforms;

use platforms::{AuthData, AuthService};
use tauri::Emitter;

#[tauri::command]
async fn authenticate(app_handle: tauri::AppHandle, platform: &str) -> Result<AuthData, String> {
    AuthService::authenticate(app_handle, platform).await
}

#[tauri::command]
async fn emit_api_data(app_handle: tauri::AppHandle, payload: &str) -> Result<(), ()> {
    let _ = app_handle.emit("hyprlance:contra-data", payload);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![authenticate, emit_api_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
