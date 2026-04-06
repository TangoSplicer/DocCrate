#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use tauri::command;
// Import your actual core library!
use doccrate_core;

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

// THE REAL ENGINE BRIDGE
// Notice this is async. This ensures the UI doesn't freeze while Rust does the heavy lifting!
#[command]
async fn build_library() -> Result<String, String> {
    let sources = get_staged_sources();
    if sources.is_empty() {
        return Err("Queue is empty. Please add sources first!".to_string());
    }

    // Call your actual Phase 4 core logic here!
    // We run it inside tokio's spawn_blocking so it doesn't block Tauri's main thread
    let result = tokio::task::spawn_blocking(move || {
        // NOTE: Replace 'build_offline_pack' with whatever you named your 
        // main compiling function in crates/core/src/lib.rs during Phase 4
        doccrate_core::build_offline_pack(&sources)
    })
    .await
    .map_err(|e| format!("Thread execution failed: {}", e))?;

    // Handle the result from your core engine
    match result {
        Ok(pack_path) => Ok(format!("Successfully built: {}", pack_path)),
        Err(e) => Err(format!("Engine Error: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_staged_sources, 
            add_source,
            build_library
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
