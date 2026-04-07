# 📦 DocCrate Studio v1.0

**The Offline Markdown Knowledge Base Publisher.**

DocCrate Studio is a cross-platform, natively compiled desktop application designed to solve a specific problem: taking scattered, online developer documentation (Git repositories, Markdown files) and packing them into a single, highly compressed, fully searchable offline archive (`.docpack`). 

Perfect for air-gapped environments, compliance officers, and offline field engineers.

---

## 🚀 Features

* **Dual-Mode Interface:** Seamlessly switch between Publisher Mode (to build archives) and Reader Mode (to view them).
* **Native Rust Engine:** Powered by a lightning-fast Rust backend that handles Git cloning, filesystem parsing, and zip archiving without freezing the UI.
* **Offline Global Search:** Linearly scan thousands of compressed Markdown files in milliseconds.
* **Beautiful Rendering:** Built-in Rust Markdown-to-HTML parsing ensures your raw `.md` files look like premium web documentation.
* **Cross-Platform:** Compiles to Windows (`.exe`) and Linux (`.AppImage`) with zero C-dependency nightmares.

---

## 📖 User Guide

### 🛠️ Publisher Mode (Building a Pack)
1. Launch DocCrate Studio and ensure you are on the **Publisher Mode** tab.
2. In the input box, paste the URL of a public Git repository (e.g., `https://github.com/rust-lang/book.git`).
3. Click **+ Add to Queue**. You can stage as many repositories as you need.
4. Click **🚀 Build & Pack Library**.
5. The engine will shallow-clone the repositories, hunt down every `.md` file, compress them, and output a `Master_Library_v1.docpack` file in the application directory.

### 📖 Reader Mode (Viewing a Pack)
1. Switch to the **Reader Mode** tab.
2. Click **📂 Select .docpack File** and locate your generated `.docpack` archive.
3. The sidebar will instantly populate with an index of all contained documents.
4. Click any document to extract it from the archive and render it in the reading pane.
5. **Search:** Use the search bar above the file list to instantly scan the entire archive for keywords. Press `Enter` to execute the search.

---

## 🏗️ Architecture

DocCrate is built on the **Tauri** framework, utilizing a multi-crate Rust workspace:
* **Frontend (`/ui`):** A lightweight, dependency-free HTML/CSS/JS interface using Flexbox for responsiveness.
* **Backend Bridge (`crates/desktop`):** The Tauri application shell that handles OS-level windowing and exposes async commands.
* **Core Engine (`crates/core`):** The heavy-lifting Rust library utilizing `tokio` (threading), `zip` (archiving), and `pulldown-cmark` (Markdown parsing). System native `git` commands are used to bypass C-binding compilation issues on Windows.

---

## 💻 Building from Source

This repository relies on GitHub Actions for automated CI/CD cross-compilation. If you wish to build locally:

1. Ensure you have Rust, Node.js, and standard C-build tools installed for your OS.
2. Ensure you have the standard command-line `git` installed and accessible in your system PATH.
3. Navigate to `crates/desktop` and run:
   ```bash
   cargo tauri build
© 2026 DocCrate. All rights reserved.
