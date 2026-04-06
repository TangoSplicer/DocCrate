use anyhow::{Context, Result};
use git2::Repository;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::ZipWriter;

/// The public bridge function called by the Tauri Desktop App
pub fn build_offline_pack(sources: &Vec<String>) -> Result<String, String> {
    // We map the internal anyhow::Result to a simple String error 
    // so the React frontend can easily display it in the UI if it fails.
    match run_engine(sources) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(format!("Engine Failure: {:#}", e)),
    }
}

/// The internal heavy-lifting engine
fn run_engine(sources: &Vec<String>) -> Result<String> {
    if sources.is_empty() {
        anyhow::bail!("No sources provided in the queue.");
    }

    let stage_dir = Path::new("doccrate_staging");
    
    // 1. Clean slate: remove old staging dir if it exists
    if stage_dir.exists() {
        fs::remove_dir_all(stage_dir).context("Failed to clear previous staging directory")?;
    }
    fs::create_dir_all(stage_dir).context("Failed to create staging directory")?;

    // 2. Clone all repositories
    for (i, url) in sources.iter().enumerate() {
        // Name them repo_0, repo_1, etc. to avoid naming collisions
        let repo_dir = stage_dir.join(format!("repo_{}", i));
        Repository::clone(url, &repo_dir).context(format!("Failed to clone Git URL: {}", url))?;
    }

    // 3. Prepare the Output Archive (.docpack is just a custom ZIP)
    let output_file = "Master_Library_v1.docpack";
    let file = File::create(output_file).context("Failed to create output archive file")?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default();

    // 4. Hunt for Markdown files and pack them
    let mut packed_count = 0;
    
    // WalkDir recursively searches every folder inside our staging directory
    for entry in WalkDir::new(stage_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // If it's a file AND ends in .md
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            // Create a clean relative path for the zip file internal structure
            let name = path.strip_prefix(stage_dir).unwrap_or(path);
            let zip_path = name.to_string_lossy().into_owned();
            
            // Start a new file entry in the zip
            zip.start_file(zip_path, options).context("Failed to start zip entry")?;
            
            // Read the actual markdown file from the disk
            let mut f = File::open(path).context("Failed to open markdown file")?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            
            // Write it into the zip archive
            zip.write_all(&buffer).context("Failed to write markdown data to zip")?;
            
            packed_count += 1;
        }
    }

    // 5. Finalize and save the archive
    zip.finish().context("Failed to finalize the docpack archive")?;

    // 6. Cleanup the massive uncompressed git repos to save disk space
    let _ = fs::remove_dir_all(stage_dir);

    // 7. Return the success message to the UI!
    Ok(format!("Built {} containing {} Markdown files!", output_file, packed_count))
}
