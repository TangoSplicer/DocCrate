use clap::{Parser, Subcommand};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;

const STAGE_FILE: &str = "doccrate.json";

#[derive(Parser)]
#[command(name = "doccrate", about = "Offline Documentation Builder CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a source (URL or folder) to the staging area
    Add { source: String },
    /// Show currently staged sources
    Status,
    /// Build documentation from staged sources (or a specific source)
    Build {
        #[arg(short, long)]
        source: Option<String>,
        #[arg(short, long, default_value = "./dist")]
        out: String,
    },
    /// Compress a built documentation directory into a portable .docpack
    Pack {
        #[arg(short, long, default_value = "./dist")]
        source: String,
        #[arg(short, long, default_value = "Library.docpack")]
        out: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Add { source } => add_source(source),
        Commands::Status => show_status(),
        Commands::Build { source, out } => {
            let out_path = Path::new(out);
            let _ = fs::create_dir_all(out_path);

            if let Some(s) = source {
                // Build just one source
                build_single_docs(s, out_path);
            } else {
                // Build everything in the staging file
                let sources = get_staged_sources();
                if sources.is_empty() {
                    println!("⚠️ No sources staged. Use 'doccrate add <source>' first.");
                    return;
                }
                println!("🚀 Building {} staged sources...", sources.len());
                for s in &sources {
                    build_single_docs(s, out_path);
                }
                generate_master_index(out_path);
                println!("🎉 Master Library build complete!");
            }
        },
        Commands::Pack { source, out } => pack_docs(source, out),
    }
}

// --- STAGING LOGIC ---
fn get_staged_sources() -> Vec<String> {
    if let Ok(content) = fs::read_to_string(STAGE_FILE) {
        if let Ok(sources) = serde_json::from_str::<Vec<String>>(&content) {
            return sources;
        }
    }
    Vec::new()
}

fn add_source(source: &str) {
    let mut sources = get_staged_sources();
    if !sources.contains(&source.to_string()) {
        sources.push(source.to_string());
        let json = serde_json::to_string_pretty(&sources).unwrap();
        fs::write(STAGE_FILE, json).unwrap();
        println!("✅ Added to staging: {}", source);
    } else {
        println!("ℹ️ Source is already staged.");
    }
}

fn show_status() {
    let sources = get_staged_sources();
    println!("🗂️  Currently Staged Sources:");
    if sources.is_empty() {
        println!("   (None)");
    } else {
        for (i, s) in sources.iter().enumerate() {
            println!("   {}. {}", i + 1, s);
        }
    }
}

// --- MASTER INDEX LOGIC ---
fn generate_master_index(out_dir: &Path) {
    println!("🎨 Generating Master Library Index...");
    let mut html = String::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DocCrate Library</title>
    <style>
        :root { --bg: #f6f8fa; --card: #ffffff; --border: #d0d7de; --text: #24292f; --link: #0969da; --hover: #f3f4f6; }
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif; line-height: 1.6; margin: 0; padding: 2rem 1rem; color: var(--text); background-color: var(--bg); }
        .container { max-width: 900px; margin: 0 auto; background-color: var(--card); padding: 2.5rem; border-radius: 12px; box-shadow: 0 4px 14px rgba(0,0,0,0.05); border: 1px solid var(--border); }
        h1 { border-bottom: 2px solid var(--border); padding-bottom: 0.3em; margin-top: 0; }
        p { color: #57606a; font-size: 1.1em; }
        .folder-list { list-style: none; padding: 0; border: 1px solid var(--border); border-radius: 8px; overflow: hidden; margin-top: 1.5rem; }
        .folder-list li { border-bottom: 1px solid var(--border); }
        .folder-list li:last-child { border-bottom: none; }
        .folder-list a { display: flex; align-items: center; gap: 10px; padding: 16px; color: var(--text); text-decoration: none; font-weight: 600; font-size: 1.1em; transition: background 0.2s; }
        .folder-list a:hover { background-color: var(--hover); color: var(--link); }
    </style>
</head>
<body>
    <div class="container">
        <h1>📚 Offline Knowledge Library</h1>
        <p>Welcome to your completely offline documentation bundle. Select a project below to browse its files:</p>
        <ul class="folder-list">"#);

    if let Ok(entries) = fs::read_dir(out_dir) {
        let mut folders: Vec<String> = entries.filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        folders.sort();
        for folder in folders {
            html.push_str(&format!("\n            <li><a href=\"{}/index.html\">📁 {}</a></li>", folder, folder));
        }
    }

    html.push_str("\n        </ul>\n    </div>\n</body>\n</html>");
    fs::write(out_dir.join("index.html"), html).unwrap();
}

// --- BUILD CORE LOGIC ---
fn build_single_docs(source: &str, base_out_path: &Path) {
    let mut actual_source = source.to_string();
    let temp_dir = "./.doccrate_temp";

    let project_name = if source.starts_with("http") || source.starts_with("git@") {
        source.split('/').last().unwrap_or("unknown").replace(".git", "")
    } else {
        Path::new(&source).file_name().unwrap_or_default().to_string_lossy().into_owned()
    };

    println!("🔄 Processing: {}", project_name);
    let out_path = base_out_path.join(&project_name);

    if source.starts_with("http") || source.starts_with("git@") {
        let _ = fs::remove_dir_all(temp_dir);
        let status = Command::new("git").args(["clone", "--depth", "1", "-q", source, temp_dir]).status();
        if status.is_err() || !status.unwrap().success() {
            eprintln!("❌ Failed to clone {}. Skipping.", source);
            return;
        }
        actual_source = temp_dir.to_string();
    }

    let source_path = Path::new(&actual_source);
    if !source_path.exists() {
        eprintln!("❌ Source not found: {}", actual_source);
        return;
    }
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
}

fn pack_docs(source: &str, out: &str) {
    println!("📦 Packaging documentation bundle...");
    let source_path = Path::new(source);
    if !source_path.exists() || !source_path.is_dir() {
        eprintln!("Error: Source directory '{}' does not exist.", source);
        return;
    }
    let status = Command::new("zip").current_dir(source).args(["-r", "-q", &format!("../{}", out), "."]).status().unwrap();
    if status.success() { println!("✅ Successfully created portable bundle: {}", out); } 
    else { eprintln!("❌ Failed to package the bundle."); }
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
                                json_list.push(json!({ "path": web_link, "title": title, "content": content }));
                            }
                        }
                    }
                }
            }
        }
    }
}
