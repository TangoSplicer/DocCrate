# 🗃️ DocCrate Core

DocCrate is a high-performance, zero-dependency offline documentation builder written in Rust. It compiles remote GitHub repositories and local folders into highly polished, searchable, and fully offline HTML knowledge bundles (`.docpack`). 

Built for secure environments, air-gapped systems, and premium knowledge distribution, DocCrate ensures that technical documentation remains accessible anywhere, without requiring a server or internet connection.

## ✨ Core Architecture & Features

* **Zero-Dependency Viewing:** Generated bundles require no internet connection, external CDNs, or backend databases to view. Everything is embedded.
* **Staging Area (Multi-Repo Federation):** Combine multiple repositories into a single "Master Library" dashboard using a managed staging queue (`doccrate.json`).
* **Offline Fuzzy Search:** Features instant, typo-tolerant search. During the build phase, DocCrate safely escapes and compresses content into a local `search_index.json` file, which is parsed by an embedded Vanilla JS engine at runtime.
* **Smart Parsing & Code Wrapping:** Automatically detects over 15 code extensions (Rust, Python, JS, TS, etc.) and wraps them in syntax-highlighted Markdown code blocks.
* **Native File Explorer:** Replicates the exact directory tree of the source material using native HTML5 `<details>` and `<summary>` tags for lightweight, JavaScript-free collapsible folder navigation.
* **Fast Native Bindings:** Leverages the host system's `git` and `zip` utilities directly to bypass heavy C-bindings and ensure ultra-fast compilation on mobile environments like Termux.

## 🚀 Technical Usage

DocCrate is driven via a standard CLI interface.

### 1. Staging Sources
Instead of building one repository at a time, DocCrate uses a staging system to build comprehensive libraries.
```bash
cargo run -p doccrate-cli -- add [https://github.com/TangoSplicer/project-aletheia.git](https://github.com/TangoSplicer/project-aletheia.git)
cargo run -p doccrate-cli -- add ./local_secret_notes
cargo run -p doccrate-cli -- status
### 2. Building the Library
​Once sources are staged, run the build command to clone, parse, and generate the Master Index.                              ```bash                                                       cargo run -p doccrate-cli -- build --out ./dist
### 3. Packaging
​Compress the generated HTML into a portable bundle.           ```bash                                                       cargo run -p doccrate-cli -- pack --source ./dist --out My_Premium_Library.docpack
📜 License & Compliance
​Designed to support frameworks like the UK FSR, NIST, and the EU AI Act by providing immutable, offline snapshots of compliance documentation and codebases.
