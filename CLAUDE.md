# Wizard App

Tauri v2 desktop app that wraps an external web app at `http://wizard.local:9000/` as a native macOS application. Features a settings window, native menu bar, and external link handling.

## Prerequisites

- Rust (edition 2021): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Tauri CLI v2: `cargo install tauri-cli --version "^2"`
- just (optional): `brew install just`
- The external service at `http://wizard.local:9000/` must be running

## Commands

```bash
# Using just (from repo root)
just dev          # Development mode
just build        # Release build
just install      # Build + copy to /Applications
just uninstall    # Remove from /Applications

# Manual (from repo root)
cd src-tauri && cargo tauri dev
cd src-tauri && cargo tauri build --bundles app

# Custom default URL at build time
cd src-tauri && DEFAULT_ENDPOINT_URL="http://myhost:9000/" cargo tauri build --bundles app
```

## Architecture

This is a pure Rust/Tauri project (no Node.js). The app creates a native macOS window that loads an external URL — there is no bundled frontend beyond a minimal loading page and settings UI.

- `src-tauri/src/lib.rs` — App entry point; builds native menu bar, creates main webview window, opens settings window, handles external links
- `src-tauri/src/main.rs` — Binary entry point (calls `lib::run()`)
- `src-tauri/src/commands.rs` — Tauri IPC commands: get/set endpoint URL, close settings window
- `src-tauri/src/settings.rs` — Settings persistence (JSON file in app data dir); compile-time default via `DEFAULT_ENDPOINT_URL` env var
- `src-tauri/tauri.conf.json` — Tauri config (app name, identifier, `withGlobalTauri: true`)
- `src-tauri/capabilities/default.json` — Tauri v2 capability permissions (main + settings windows)
- `ui/index.html` — Minimal loading placeholder shown briefly before redirect
- `ui/settings.html` — Settings UI (endpoint URL form with save/cancel)

## Customization

- **Default endpoint URL:** Set `DEFAULT_ENDPOINT_URL` env var at build time, or change the fallback in `src-tauri/src/settings.rs`
- **Runtime endpoint URL:** Settings window (Cmd+,) — persisted to app data dir
- **Window size:** Edit `inner_size()` in `src-tauri/src/lib.rs` (default: 1280x800)
- **App icon:** Replace files in `src-tauri/icons/` or run `cargo tauri icon your-image.png`
- **App name / identifier:** `src-tauri/tauri.conf.json`
- **Variant builds:** Create a config overlay in `src-tauri/variants/` and build with `--config variants/myvariant.json`
