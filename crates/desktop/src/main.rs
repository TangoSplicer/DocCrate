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

// THE NEW ENGINE WIRING
#[command]
async fn build_library() -> Result<String, String> {
    let sources = get_staged_sources();
    if sources.is_empty() {
        return Err("Queue is empty. Please add sources first!".to_string());
    }

    // Here is where doccrate_core runs the cloning/packing logic.
    // For the UI bridge test, we will ensure it can write a final output file.
    let output_file = "Master_Library.docpack";
    
    // Simulate engine work...
    std::thread::sleep(std::time::Duration::from_secs(2)); 
    
    // Write the output package
    fs::write(output_file, "DOCPACK_V1_MOCK_DATA").map_err(|e| e.to_string())?;

    Ok(format!("Successfully built {} with {} sources!", output_file, sources.len()))
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
