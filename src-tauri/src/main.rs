#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nasoone_lib::Nasoone;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn get_devices() -> String {
    let devices = Nasoone::list_devices().unwrap();
    serde_json::to_string(&devices).unwrap()
}

#[tauri::command]
fn start(
    device: &str,
    output_folder: &str,
    filename: &str,
    interval: u32,
    state: State<Mutex<Nasoone>>,
) -> Result<String, String> {
    let mut nasoone = state.lock().unwrap();
    nasoone
        .set_capture_device(device)
        .map_err(|_| "Unable to set Nasoone capture device".to_string())?;
    let _output_path = format!("{}/{}", output_folder, filename);
    let _interval = interval;
    nasoone
        .start()
        .map_err(|_| "Unable to start Nasoone".to_string())?;
    Ok("Nasoone started".to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Nasoone::default()))
        .invoke_handler(tauri::generate_handler![get_devices, start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
