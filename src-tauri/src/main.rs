#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use nasoone_lib::{ Nasoone };

#[tauri::command]
fn get_devices() -> String {
  let devices = Nasoone::list_devices().unwrap();
  serde_json::to_string(&devices).unwrap()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_devices])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
