#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri_plugin_fs_extra::FsExtra;
use std::path::Path;
use nasoone_lib::{Nasoone};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn get_devices() -> Result<String, String> {
    let devices = Nasoone::list_devices().map_err(|e| e.to_string())?;
    let devices = devices.into_iter().map(|d| d.to_string()).collect::<Vec<_>>();
    Ok(serde_json::to_string(&devices).unwrap())
}

#[tauri::command]
fn pause(state: State<Mutex<Nasoone>>) -> Result<String, String> {
    let mut nasoone = state.lock().unwrap();
    nasoone.pause().map_err(|e| e.to_string())?;
    Ok("Nasoone paused".to_string())
}

#[tauri::command]
fn reset(state: State<Mutex<Nasoone>>) -> Result<String, String> {
    let new_naso = Nasoone::default();
    let mut nasoone = state.lock().unwrap();
    *nasoone = new_naso;
    Ok("Nasoone resetted".to_string())
}

#[tauri::command]
fn stop(state: State<Mutex<Nasoone>>) -> Result<String, String> {
    let mut nasoone = state.lock().unwrap();
    nasoone.stop().map_err(|e| e.to_string())?;
    Ok("Nasoone stopped".to_string())
}

#[tauri::command]
fn resume(state: State<Mutex<Nasoone>>) -> Result<String, String> {
    let mut nasoone = state.lock().unwrap();
    nasoone.resume().map_err(|e| e.to_string())?;
    Ok("Nasoone resumed".to_string())
}

#[tauri::command]
fn get_total_packets(state: State<Mutex<Nasoone>>) -> usize {
    let mut nasoone = state.lock().unwrap();
    nasoone.get_total_packets()
}

#[tauri::command]
fn start(
    device: &str,
    output_folder: &str,
    filename: &str,
    interval: u32,
    filter: &str,
    state: State<Mutex<Nasoone>>,
) -> Result<String, String> {
    let mut nasoone = state.lock().unwrap();
    let output_path = Path::new(output_folder).join(filename);
    nasoone
        .set_capture_device(device)
        .map_err(|_| "Unable to set Nasoone capture device".to_string())?;
    nasoone
        .set_output(output_path.into_os_string().into_string().unwrap().as_str())
        .map_err(|_| "Unable to set Nasoone output path".to_string())?;
    nasoone
        .set_timeout(interval)
        .map_err(|_| "Unable to set Nasoone timeout".to_string())?;
    if !filter.is_empty() {
        nasoone
            .set_filter(filter)
            .map_err(|_| "Invalid BPF filter".to_string())?;
    }
    nasoone
        .start()
        .map_err(|_| "Unable to start Nasoone".to_string())?;
    Ok("Nasoone started".to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(FsExtra::default())
        .manage(Mutex::new(Nasoone::default()))
        .invoke_handler(tauri::generate_handler![get_devices, start, pause, stop, resume, reset, get_total_packets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
