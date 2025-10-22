use crate::{serial::driver::check_available_ports, state::AppState};

#[tauri::command]
pub fn list_ports() -> Vec<String> {
    let ports = check_available_ports();
    ports
}

#[tauri::command]
pub fn open_serial(
    state: tauri::State<AppState>,
    window: tauri::Window,
    port: String,
    baud: u32,
) -> Result<(), String> {
    state
        .serial
        .lock()
        .unwrap()
        .open(&port, baud, window)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_serial(state: tauri::State<AppState>, data: String) -> Result<(), String> {
    state
        .serial
        .lock()
        .unwrap()
        .write(&data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn close_serial(state: tauri::State<AppState>) -> Result<(), String> {
    state.serial.lock().unwrap().close();
    Ok(())
}

#[tauri::command]
pub fn change_settings(
    state: tauri::State<AppState>,
    window: tauri::Window,
    port: String,
    baud: u32,
) -> Result<(), String> {
    state
        .serial
        .lock()
        .unwrap()
        .change_settings(&port, baud, window)
        .map_err(|e| e.to_string())
}
