#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;
use nasoone_lib::{Filter, Nasoone};
use std::sync::Mutex;
use tauri::State;


#[tauri::command]
fn get_bpf_filter_from_easy_filter(src_ips: Vec<&str>, dst_ips: Vec<&str>, src_ports: Vec<u16>, dst_ports: Vec<u16>, tcp: bool, udp: bool) -> Result<String, String> {
    let mut filter = Filter::default();
    for ip in src_ips {
        filter = filter.add_src_host(IpAddr::from_str(ip).map_err(|e| e.to_string())?);
    }
    for ip in dst_ips {
        filter = filter.add_dst_host(IpAddr::from_str(ip).map_err(|e| e.to_string())?);
    }
    for port in src_ports {
        filter = filter.add_src_port(port);
    }
    for port in dst_ports {
        filter = filter.add_dst_port(port);
    }
    if tcp && !udp {
        filter = filter.set_tcp_only();
    }
    if !tcp && udp {
        filter = filter.set_udp_only();
    }
    Ok(filter.to_string())
}

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
            .set_raw_filter(filter)
            .map_err(|_| "Invalid BPF filter".to_string())?;
    }
    nasoone
        .start()
        .map_err(|_| "Unable to start Nasoone".to_string())?;
    Ok("Nasoone started".to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(Nasoone::default()))
        .invoke_handler(tauri::generate_handler![get_devices, start, pause, stop, resume, reset, get_total_packets, get_file_size, get_bpf_filter_from_easy_filter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
