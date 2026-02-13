# Wizard App

Minimal Tauri v2 desktop app that wraps an external web app at `http://wizard.local:9000/` as a native macOS application.

## Prerequisites

- Rust (edition 2021): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Tauri CLI v2: `cargo install tauri-cli --version "^2"`
- The external service at `http://wizard.local:9000/` must be running

## Commands

```bash
# Development (from repo root)
cd src-tauri && cargo tauri dev

# Release build (produces .app bundle in src-tauri/target/release/bundle/macos/)
cd src-tauri && cargo tauri build
```

## Architecture

This is a pure Rust/Tauri project (no Node.js). The app creates a native macOS window that loads an external URL — there is no bundled frontend beyond a minimal loading page.

- `src-tauri/src/lib.rs` — App entry point; creates the webview window pointing at the external URL
- `src-tauri/src/main.rs` — Binary entry point (calls `lib::run()`)
- `src-tauri/tauri.conf.json` — Tauri config (app name, identifier, security permissions for remote URL)
- `src-tauri/capabilities/default.json` — Tauri v2 capability permissions
- `ui/index.html` — Minimal loading placeholder shown briefly before redirect

## Customization

- **Target URL:** Change the URL string in `src-tauri/src/lib.rs`
- **Window size:** Edit `inner_size()` in `src-tauri/src/lib.rs` (default: 1280x800)
- **App icon:** Replace files in `src-tauri/icons/` or run `cargo tauri icon your-image.png`
- **Security/capabilities:** Configured in `src-tauri/capabilities/default.json` (Tauri v2 capabilities system)
