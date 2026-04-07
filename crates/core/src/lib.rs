use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;
use pulldown_cmark::{Parser, html};

pub fn build_offline_pack(sources: &Vec<String>) -> Result<String, String> {
    match run_engine(sources) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(format!("Engine Failure: {:#}", e)),
    }
}

fn run_engine(sources: &Vec<String>) -> Result<String> {
    if sources.is_empty() { anyhow::bail!("No sources provided in the queue."); }
    let stage_dir = Path::new("doccrate_staging");
    if stage_dir.exists() { fs::remove_dir_all(stage_dir).context("Failed to clear previous staging directory")?; }
    fs::create_dir_all(stage_dir).context("Failed to create staging directory")?;
    
    for (i, url) in sources.iter().enumerate() {
        let repo_dir = stage_dir.join(format!("repo_{}", i));
        let status = std::process::Command::new("git")
            .arg("clone").arg("--depth").arg("1").arg(url).arg(&repo_dir)
            .status().context(format!("Failed to execute native git clone for: {}", url))?;
        if !status.success() { anyhow::bail!("Git clone failed for URL: {}", url); }
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

pub fn list_docpack_files(pack_path: &str) -> Result<Vec<String>, String> {
    let file = File::open(pack_path).map_err(|e| format!("Could not open file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Not a valid DocCrate pack: {}", e))?;
    let mut files = Vec::new();
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) { files.push(file.name().to_string()); }
    }
    Ok(files)
}

pub fn read_docpack_file(pack_path: &str, target_file: &str) -> Result<String, String> {
    let file = File::open(pack_path).map_err(|e| format!("Could not open file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Not a valid DocCrate pack: {}", e))?;
    let mut inner_file = archive.by_name(target_file).map_err(|e| format!("File not found in pack: {}", e))?;
    let mut raw_markdown = String::new();
    inner_file.read_to_string(&mut raw_markdown).map_err(|e| format!("Failed to read file: {}", e))?;
    
    let parser = Parser::new(&raw_markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output)
}

pub fn search_docpack(pack_path: &str, query: &str) -> Result<Vec<String>, String> {
    let file = File::open(pack_path).map_err(|e| format!("Could not open file: {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Not a valid DocCrate pack: {}", e))?;
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    for i in 0..archive.len() {
        if let Ok(mut inner_file) = archive.by_index(i) {
            if inner_file.is_file() {
                let name = inner_file.name().to_string();
                if name.ends_with(".md") {
                    let mut contents = String::new();
                    if inner_file.read_to_string(&mut contents).is_ok() {
                        if contents.to_lowercase().contains(&query_lower) {
                            results.push(name);
                        }
                    }
                }
            }
        }
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_engine_rejects_empty_queue() {
        let empty_sources = Vec::new();
        let result = build_offline_pack(&empty_sources);
        assert!(result.is_err());
    }
}
