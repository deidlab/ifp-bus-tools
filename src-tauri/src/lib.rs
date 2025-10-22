mod serial;
mod state;

use serial::api::*;
use tauri::Manager;

use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState::default());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_ports,
            open_serial,
            close_serial,
            write_serial,
            change_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
