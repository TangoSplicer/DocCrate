# 🗃️ DocCrate Core

DocCrate is a high-performance, zero-dependency offline documentation builder. It transforms raw GitHub repositories and local folders into highly polished, searchable, and fully offline HTML knowledge bundles (`.docpack`).

## ✨ Features
- **Zero-Dependency Viewing:** Generated bundles require no internet connection, external CSS, or databases to view.
- **Offline Fuzzy Search:** Instant, typo-tolerant search powered by a highly compressed, pre-compiled JSON index.
- **Multi-Repo Federation:** Combine multiple repositories into a single "Master Library" dashboard.
- **Smart Formatting:** Automatically detects code files (Rust, Python, JS, etc.) and wraps them in beautiful syntax-highlighted markdown.
- **Folder Preservation:** Replicates the exact directory tree of the source material using native HTML5 collapsible sections.

## 🚀 Quick Start (CLI)
DocCrate is built natively in Rust. To build a project:
```bash
cargo run -p doccrate-cli -- build --source [https://github.com/username/repo.git](https://github.com/username/repo.git) --out ./dist
cargo run -p doccrate-cli -- pack --source ./dist --out MyLibrary.docpack
For detailed instructions on creating commercial bundles, please see the User Guide.
