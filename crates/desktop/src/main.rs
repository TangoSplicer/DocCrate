#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use tauri::command;

const STAGE_FILE: &str = "doccrate.json";

#[command]
fn get_staged_sources() -> Vec<String> {
    if let Ok(content) = fs::read_to_string(STAGE_FILE) {
        if let Ok(sources) = serde_json::from_str::<Vec<String>>(&content) {
            return sources;
        }
    }
    Vec::new()
}

#[command]
fn add_source(source: String) -> Result<Vec<String>, String> {
    let mut sources = get_staged_sources();
    if !sources.contains(&source) && !source.trim().is_empty() {
        sources.push(source);
        let json = serde_json::to_string_pretty(&sources).map_err(|e| e.to_string())?;
        fs::write(STAGE_FILE, json).map_err(|e| e.to_string())?;
    }
    Ok(get_staged_sources())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_staged_sources, add_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
