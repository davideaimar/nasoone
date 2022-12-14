#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;
use std::str::FromStr;
use nasoone_lib::{Nasoone};
use nasoone_lib::{Filter};
use std::sync::Mutex;
use tauri::State;
use std::net::IpAddr;

#[tauri::command]
fn get_devices() -> Result<String, String> {
    let devices = Nasoone::list_devices().map_err(|e| e.to_string())?;
    let devices = devices.into_iter().map(|d| d.to_string()).collect::<Vec<_>>();
    Ok(serde_json::to_string(&devices).unwrap())
}

#[tauri::command]
fn get_file_size(path: &str) -> Result<u64, String> {
    let meta = std::fs::metadata(path).map_err(|_| "Failed to get file size".to_string())?;
    Ok(meta.len())
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

#[tauri::command]
fn new_filter() -> Filter {
    Filter::default()
}

#[tauri::command]
fn add_host(fil: Filter, host: &str, not: bool) -> Filter {
    let new_filter = fil.add_host(IpAddr::from_str(host), not);
    new_filter
}

#[tauri::command]
fn add_src_host(fil: Filter, src_host: &str, not: bool) -> Filter {
    let new_filter = fil.add_src_host(IpAddr::from_str(src_host), not);
    new_filter
}

#[tauri::command]
fn add_dst_host(fil: Filter, dst_host: &str, not: bool) -> Filter {
    let new_filter = fil.add_dst_host(IpAddr::from_str(dst_host), not);
    new_filter
}

#[tauri::command]
fn set_ether_host(fil: Filter, ether_host: &str, not: bool) -> Filter {
    let new_filter = fil.set_eather_host(ether_host, not);
    new_filter
}

#[tauri::command]
fn set_ether_src_host(fil: Filter, ether_src_host: &str, not: bool) -> Filter {
    let new_filter = fil.set_eather_src_host(ether_src_host, not);
    new_filter
}

#[tauri::command]
fn set_ether_dst_host(fil: Filter, ether_dst_host: &str, not: bool) -> Filter {
    let new_filter = fil.set_eather_dst_host(ether_dst_host, not);
    new_filter
}

#[tauri::command]
fn set_protocols(fil: Filter, proto: Vec<&str>) -> Result<Filter, ()> {
    proto.into_iter().for_each(| p | {
        match p {
            "http" => fil.set_http(true),
            "tcp" => fil.set_tcp(true),
            "smtp" => fil.set_smtp(true),
            "dns" => fil.set_dns(true),
            "udp" => fil.set_udp(true),
            _ => Err(())
        }
    });
    return Ok(fil);
}

#[tauri::command]
fn set_port(fil: Filter, port: i32) -> Filter {
    let new_filter = fil.set_port(port);
    new_filter
}

#[tauri::command]
fn set_src_port(fil: Filter, src_port: i32) -> Filter {
    let new_filter = fil.set_src_port(src_port);
    new_filter
}

#[tauri::command]
fn set_dst_port(fil: Filter, dst_port: i32) -> Filter {
    let new_filter = fil.set_dst_port(dst_port);
    new_filter
}

#[tauri::command]
fn get_BPF_filter(fil: Filter) -> String {
    return fil.toString()
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Nasoone::default()))
        .invoke_handler(tauri::generate_handler![get_devices, start, pause, stop, resume, reset, get_total_packets, get_file_size])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
