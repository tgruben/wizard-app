# Wizard App

A native macOS desktop wrapper for the Wizard web application. Built with [Tauri v2](https://v2.tauri.app/), it loads `http://wizard.local:9000/` in a lightweight native window — no Electron, no bundled browser engine.

## Features

- **Settings window** (Cmd+,) — change the endpoint URL at runtime; persisted to disk
- **Native menu bar** — standard macOS app, Edit, View, and Window menus
- **External link handling** — links targeting new windows open in the default browser
- **Compile-time default URL** — set `DEFAULT_ENDPOINT_URL` env var at build time to override the default endpoint

## Prerequisites

- **Rust** (2021 edition): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Tauri CLI v2**: `cargo install tauri-cli --version "^2"`
- **just** (optional, for convenience): `brew install just`
- The Wizard service running at `http://wizard.local:9000/`

## Getting Started

### Using just (recommended)

```bash
just dev        # Development mode with hot-reload
just build      # Release build (.app bundle)
just install    # Build and copy to /Applications
just uninstall  # Remove from /Applications
```

### Manual

```bash
cd src-tauri
cargo tauri dev                    # Development
cargo tauri build --bundles app    # Release build
```

The `Wizard.app` bundle will be at `src-tauri/target/release/bundle/macos/Wizard.app`.

### Custom default URL

Set `DEFAULT_ENDPOINT_URL` at compile time to change the built-in default:

```bash
DEFAULT_ENDPOINT_URL="http://myhost:9000/" cargo tauri build --bundles app
```

The `just` recipes already set this (see `wizard_url` in the justfile).

## Project Structure

```
wizard-app/
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs          # App setup — menus, settings window, external link handling
│   │   ├── main.rs          # Binary entry point
│   │   ├── commands.rs      # Tauri commands (get/set endpoint URL, close settings)
│   │   └── settings.rs      # Settings persistence (JSON in app data dir)
│   ├── capabilities/
│   │   └── default.json     # Tauri v2 permissions (main + settings windows)
│   ├── icons/                # App icons (all platforms)
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # Tauri config (app name, identifier, withGlobalTauri)
├── ui/
│   ├── index.html            # Loading placeholder shown briefly on startup
│   └── settings.html         # Settings UI (endpoint URL form)
└── justfile                  # Build/install recipes
```

## Customization

| What | Where |
|------|-------|
| Default endpoint URL | `DEFAULT_ENDPOINT_URL` env var at build time, or `src-tauri/src/settings.rs` fallback |
| Runtime endpoint URL | Settings window (Cmd+,) — persisted to app data dir |
| Window size | `src-tauri/src/lib.rs` — change `.inner_size(1280.0, 800.0)` |
| App icon | Run `cargo tauri icon your-image.png` (1024x1024 PNG recommended) |
| App name / identifier | `src-tauri/tauri.conf.json` |

## Creating a Variant Build

To create a variant (different name, icon, default URL) without forking the code:

1. Create a Tauri config overlay (e.g., `src-tauri/variants/myapp.json`) that overrides `productName`, `identifier`, and icon paths
2. Build with the overlay and a custom URL:

```bash
cd src-tauri
DEFAULT_ENDPOINT_URL="https://myapp.example.com/" cargo tauri build --bundles app --config variants/myapp.json
```
