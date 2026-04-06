use anyhow::{Context, Result};
use git2::Repository;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter}; // Added ZipArchive for reading

// --- PHASE 6: PUBLISHER LOGIC (BUILDING) ---

pub fn build_offline_pack(sources: &Vec<String>) -> Result<String, String> {
    match run_engine(sources) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(format!("Engine Failure: {:#}", e)),
    }
}

fn run_engine(sources: &Vec<String>) -> Result<String> {
    if sources.is_empty() {
        anyhow::bail!("No sources provided in the queue.");
    }

    let stage_dir = Path::new("doccrate_staging");
    if stage_dir.exists() {
        fs::remove_dir_all(stage_dir).context("Failed to clear previous staging directory")?;
    }
    fs::create_dir_all(stage_dir).context("Failed to create staging directory")?;

    for (i, url) in sources.iter().enumerate() {
        let repo_dir = stage_dir.join(format!("repo_{}", i));
        Repository::clone(url, &repo_dir).context(format!("Failed to clone Git URL: {}", url))?;
    }

    let output_file = "Master_Library_v1.docpack";
    let file = File::create(output_file).context("Failed to create output archive file")?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default();

    let mut packed_count = 0;
    
    for entry in WalkDir::new(stage_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let name = path.strip_prefix(stage_dir).unwrap_or(path);
            let zip_path = name.to_string_lossy().into_owned();
            
            zip.start_file(&zip_path, options).context("Failed to start zip entry")?;
            
            let mut f = File::open(path).context("Failed to open markdown file")?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            
            zip.write_all(&buffer).context("Failed to write markdown data to zip")?;
            packed_count += 1;
        }
    }

    zip.finish().context("Failed to finalize the docpack archive")?;
    let _ = fs::remove_dir_all(stage_dir);
    Ok(format!("Built {} containing {} Markdown files!", output_file, packed_count))
}

// --- PHASE 7: READER LOGIC (EXTRACTING) ---

/// Opens a .docpack file and returns a list of all markdown files inside it
pub fn list_docpack_files(pack_path: &str) -> Result<Vec<String>, String> {
    let file = File::open(pack_path).map_err(|e| format!("Could not open file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Not a valid DocCrate pack: {}", e))?;
    
    let mut files = Vec::new();
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            files.push(file.name().to_string());
        }
    }
    
    Ok(files)
}

/// Extracts the raw text content of a specific file inside the .docpack
pub fn read_docpack_file(pack_path: &str, target_file: &str) -> Result<String, String> {
    let file = File::open(pack_path).map_err(|e| format!("Could not open file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Not a valid DocCrate pack: {}", e))?;
    
    let mut inner_file = archive.by_name(target_file).map_err(|e| format!("File not found in pack: {}", e))?;
    
    let mut contents = String::new();
    inner_file.read_to_string(&mut contents).map_err(|e| format!("Failed to read file: {}", e))?;
    
    Ok(contents)
}
