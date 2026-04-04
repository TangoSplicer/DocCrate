# 📚 DocCrate Publisher: Comprehensive User Guide

Welcome to the DocCrate Publisher documentation. This guide explains how to curate, build, and package offline knowledge bundles.

## 1. System Requirements
DocCrate Core is optimized for fast compilation on environments like Termux (Android) or standard Linux distributions.
- **Rust Toolchain:** (`cargo`)
- **System Utilities:** `git` (for cloning) and `zip` (for packaging).

## 2. Core CLI Commands
The engine is driven by the `doccrate-cli` binary.

### `build`
Fetches a source and converts it into styled offline HTML.
- `--source`: A local directory path (e.g., `./my_notes`) or a remote Git URL (e.g., `https://github.com/...`).
- `--out`: The destination folder (defaults to `./dist`).

*Example:*
`cargo run -p doccrate-cli -- build --source ./my_notes --out ./dist`

### `pack`
Compresses your generated `./dist` folder into a distributable `.docpack` archive.
- `--source`: The folder to compress (defaults to `./dist`).
- `--out`: The final file name (e.g., `Library.docpack`).

*Example:*
`cargo run -p doccrate-cli -- pack --source ./dist --out TangoSplicer_Library.docpack`

## 3. The Multi-Repo Automation Workflow
For creating massive libraries combining multiple projects, DocCrate uses the `build_all.sh` script.

**How it works:**
1. **Cleans** the previous `./dist` directory.
2. **Iterates** through an array of repository URLs, running the `build` command for each.
3. **Generates** a Master `index.html` at the root of `./dist` linking to all processed projects.
4. **Packages** the entire library into a `.docpack` file using the `pack` command.
5. **Serves** the unpacked HTML over a local python server (`python -m http.server 8000`) so the Publisher can preview the results before distribution.

## 4. Distribution
The final `.docpack` file is a standard zip archive with a proprietary extension. Consumers can simply rename it to `.zip`, extract it, and open `index.html` in any modern web browser to access the fully offline, searchable library.
