use clap::{Parser, Subcommand};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "doccrate", about = "Offline Documentation Builder CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build documentation from a source repository or folder
    Build {
        #[arg(short, long)]
        source: String,
        #[arg(short, long, default_value = "./dist")]
        out: String,
    },
    /// Compress a built documentation directory into a portable .docpack
    Pack {
        #[arg(short, long, default_value = "./dist")]
        source: String,
        #[arg(short, long, default_value = "library.docpack")]
        out: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Build { source, out } => build_docs(source, out),
        Commands::Pack { source, out } => pack_docs(source, out),
    }
}

// --- PACK LOGIC ---
fn pack_docs(source: &str, out: &str) {
    println!("📦 Packaging documentation bundle...");
    
    let source_path = Path::new(source);
    if !source_path.exists() || !source_path.is_dir() {
        eprintln!("Error: Source directory '{}' does not exist.", source);
        return;
    }

    // Use native zip command to recursively compress the directory
    // -r: recursive, -q: quiet
    let status = Command::new("zip")
        .current_dir(source) // Run the zip command from INSIDE the dist folder
        .args(["-r", "-q", &format!("../{}", out), "."])
        .status()
        .expect("Failed to execute zip command. Is it installed?");

    if status.success() {
        println!("✅ Successfully created portable bundle: {}", out);
    } else {
        eprintln!("❌ Failed to package the bundle.");
    }
}

// --- BUILD LOGIC (Unchanged) ---
fn build_docs(source: &str, out: &str) {
    let mut actual_source = source.to_string();
    let temp_dir = "./.doccrate_temp";
    let base_out_path = Path::new(out);

    let project_name = if source.starts_with("http") || source.starts_with("git@") {
        source.split('/').last().unwrap_or("unknown").replace(".git", "")
    } else {
        Path::new(&source).file_name().unwrap_or_default().to_string_lossy().into_owned()
    };

    let out_path = base_out_path.join(&project_name);

    if source.starts_with("http") || source.starts_with("git@") {
        let _ = fs::remove_dir_all(temp_dir);
        Command::new("git").args(["clone", "--depth", "1", &source, temp_dir]).status().unwrap();
        actual_source = temp_dir.to_string();
    }

    let source_path = Path::new(&actual_source);
    fs::create_dir_all(&out_path).unwrap();

    let mut processed_files: Vec<String> = Vec::new();
    let mut json_index_entries: Vec<serde_json::Value> = Vec::new();

    process_directory(source_path, source_path, &out_path, &mut processed_files, &mut json_index_entries);
    
    processed_files.sort();
    let mut index_md = format!("# 🗃️ {} Documentation\n\n", project_name);
    let mut current_folder = String::from("___INIT___");
    let mut in_list = false;
    
    for file in &processed_files {
        let path = Path::new(file);
        let parent = path.parent().unwrap_or(Path::new("")).to_string_lossy().to_string();
        let display_folder = if parent.is_empty() { "Root Project Files".to_string() } else { format!("📁 {}", parent) };

        if display_folder != current_folder {
            if in_list { index_md.push_str("</ul></div></details>\n\n"); }
            index_md.push_str(&format!("<details class=\"folder-group\" open>\n<summary>{}</summary>\n<div class=\"file-explorer\">\n<ul>\n", display_folder));
            current_folder = display_folder;
            in_list = true;
        }
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        index_md.push_str(&format!("<li><a href=\"{}\">📄 {}</a></li>\n", file, file_name));
    }
    if in_list { index_md.push_str("</ul></div></details>\n"); }
    
    let index_html = doccrate_core::parse_markdown(&index_md, &format!("{} Index", project_name), "./");
    fs::write(out_path.join("index.html"), index_html).unwrap();

    let search_index_json = serde_json::to_string(&json_index_entries).unwrap();
    fs::write(out_path.join("search_index.json"), search_index_json).unwrap();

    if actual_source == temp_dir { let _ = fs::remove_dir_all(temp_dir); }
    println!("✅ Build complete for {}!", project_name);
}

fn process_directory(base_dir: &Path, current_dir: &Path, out_dir: &Path, file_list: &mut Vec<String>, json_list: &mut Vec<serde_json::Value>) {
    if let Ok(entries) = fs::read_dir(current_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if !name.starts_with('.') && name != "target" && name != "node_modules" && name != "dist" {
                    process_directory(base_dir, &path, out_dir, file_list, json_list);
                }
            } else if path.is_file() {
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let is_md = ext == "md" || ext == "markdown";
                let is_code = ["rs", "py", "js", "ts", "json", "toml", "yaml", "yml", "c", "cpp", "h", "sh", "txt", "html", "css", "java", "kt"].contains(&ext);

                if is_md || is_code {
                    if let Ok(rel) = path.strip_prefix(base_dir) {
                        let depth = rel.components().count() - 1;
                        let root_prefix = if depth == 0 { "./".to_string() } else { "../".repeat(depth) };

                        let mut out_file = out_dir.join(rel);
                        out_file.set_extension(format!("{}.html", ext));
                        if let Some(parent) = out_file.parent() { let _ = fs::create_dir_all(parent); }
                        
                        if let Ok(content) = fs::read_to_string(&path) {
                            let title = path.file_name().unwrap_or_default().to_string_lossy();
                            let html = if is_md { doccrate_core::parse_markdown(&content, &title, &root_prefix) } 
                                       else { doccrate_core::parse_markdown(&format!("```{}\n{}\n```", ext, content), &title, &root_prefix) };
                            
                            if fs::write(&out_file, html).is_ok() {
                                let web_link = format!("{}.html", rel.display()).replace("\\", "/");
                                file_list.push(web_link.clone());
                                json_list.push(json!({
                                    "path": web_link,
                                    "title": title,
                                    "content": content
                                }));
                            }
                        }
                    }
                }
            }
        }
    }
}
