# Wizard App

Tauri v2 desktop app that wraps an external web app at `http://wizard.local:9000/` as a native macOS application. Features a settings window, native menu bar, and external link handling.

## Prerequisites

- Rust (edition 2021): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Tauri CLI v2: `cargo install tauri-cli --version "^2"`
- just (optional): `brew install just`
- The external service at `http://wizard.local:9000/` must be running
- Python 3 + Pillow + numpy (for icon generation only): `pip install Pillow numpy`

## Commands

```bash
# Using just (from repo root)
just dev          # Development mode
just build        # Release build
just install      # Build + copy to /Applications
just uninstall    # Remove from /Applications
just generate-icons  # Regenerate ROYGBIV icon color variants

# Manual (from repo root)
cd src-tauri && cargo tauri dev
cd src-tauri && cargo tauri build --bundles app

# Custom default URL at build time
cd src-tauri && DEFAULT_ENDPOINT_URL="http://myhost:9000/" cargo tauri build --bundles app

# Custom default icon color at build time
DEFAULT_ICON_COLOR=blue just build
```

## Architecture

This is a pure Rust/Tauri project (no Node.js). The app creates a native macOS window that loads an external URL — there is no bundled frontend beyond a minimal loading page and settings UI.

- `src-tauri/src/lib.rs` — App entry point; builds native menu bar, creates main webview window, opens settings window, handles external links
- `src-tauri/src/main.rs` — Binary entry point (calls `lib::run()`)
- `src-tauri/src/commands.rs` — Tauri IPC commands: get/set endpoint URL, get/set icon color, close settings window
- `src-tauri/src/settings.rs` — Settings persistence (JSON file in app data dir); compile-time defaults via `DEFAULT_ENDPOINT_URL` and `DEFAULT_ICON_COLOR` env vars
- `src-tauri/src/icon.rs` — Dock icon manipulation; uses macOS NSApplication API to change icon at runtime
- `src-tauri/tauri.conf.json` — Tauri config (app name, identifier, `withGlobalTauri: true`)
- `src-tauri/capabilities/default.json` — Tauri v2 capability permissions (main + settings windows)
- `src-tauri/icons/colors/` — Pre-generated ROYGBIV icon variants (bundled as Tauri resources)
- `scripts/generate_icons.py` — Python script to regenerate color icon variants from the base green icon
- `ui/index.html` — Minimal loading placeholder shown briefly before redirect
- `ui/settings.html` — Settings UI (endpoint URL form and icon color picker)

## Customization

- **Default endpoint URL:** Set `DEFAULT_ENDPOINT_URL` env var at build time, or change the fallback in `src-tauri/src/settings.rs`
- **Runtime endpoint URL:** Settings window (Cmd+,) — persisted to app data dir
- **Default icon color:** Set `DEFAULT_ICON_COLOR` env var at build time (values: red, orange, yellow, green, blue, indigo, violet). Default: green
- **Runtime icon color:** Settings window (Cmd+,) — click a color swatch. Dock icon updates immediately and persists across launches
- **Window size:** Edit `inner_size()` in `src-tauri/src/lib.rs` (default: 1280x800)
- **App icon:** Replace files in `src-tauri/icons/` or run `cargo tauri icon your-image.png`
- **App name / identifier:** `src-tauri/tauri.conf.json`
- **Variant builds:** Create a config overlay in `src-tauri/variants/` and build with `--config variants/myvariant.json`
