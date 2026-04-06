#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use tauri::command;
use doccrate_core;

const STAGE_FILE: &str = "doccrate.json";

// --- PUBLISHER COMMANDS ---

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
        if let Ok(json) = serde_json::to_string_pretty(&sources) {
            let _ = fs::write(STAGE_FILE, json);
        }
    }
    Ok(get_staged_sources())
}

#[command]
async fn build_library() -> Result<String, String> {
    let sources = get_staged_sources();
    if sources.is_empty() {
        return Err("Queue is empty. Please add sources first!".to_string());
    }

    let result = tokio::task::spawn_blocking(move || {
        doccrate_core::build_offline_pack(&sources)
    })
    .await
    .map_err(|e| format!("Thread execution failed: {}", e))?;

    result
}

// --- READER COMMANDS ---

#[command]
async fn list_pack_files(path: String) -> Result<Vec<String>, String> {
    // Run in background thread to prevent UI freezing on large archives
    tokio::task::spawn_blocking(move || {
        doccrate_core::list_docpack_files(&path)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[command]
async fn read_pack_file(path: String, target: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        doccrate_core::read_docpack_file(&path, &target)
    })
    .await
    .map_err(|e| e.to_string())?
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_staged_sources, 
            add_source,
            build_library,
            list_pack_files,   // New!
            read_pack_file     // New!
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
