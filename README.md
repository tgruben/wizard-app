# Wizard App

A native macOS desktop wrapper for the Wizard web application. Built with [Tauri v2](https://v2.tauri.app/), it loads `http://wizard.local:9000/` in a lightweight native window — no Electron, no bundled browser engine.

## Prerequisites

- **Rust** (2021 edition): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Tauri CLI v2**: `cargo install tauri-cli --version "^2"`
- The Wizard service running at `http://wizard.local:9000/`

## Getting Started

### Development

```bash
cd src-tauri
cargo tauri dev
```

### Release Build

```bash
cd src-tauri
cargo tauri build --bundles app
```

The `Wizard.app` bundle will be at `src-tauri/target/release/bundle/macos/Wizard.app`. Copy it to `/Applications` to install.

## Project Structure

```
wizard-app/
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs          # App setup — creates window pointing at external URL
│   │   └── main.rs          # Binary entry point
│   ├── capabilities/
│   │   └── default.json     # Tauri v2 permissions
│   ├── icons/                # App icons (all platforms)
│   ├── Cargo.toml            # Rust dependencies
│   └── tauri.conf.json       # Tauri config (app name, identifier, frontend)
└── ui/
    └── index.html            # Loading placeholder shown briefly on startup
```

## Customization

| What | Where |
|------|-------|
| Target URL | `src-tauri/src/lib.rs` — change the `WebviewUrl::External(...)` string |
| Window size | `src-tauri/src/lib.rs` — change `.inner_size(1280.0, 800.0)` |
| App icon | Run `cargo tauri icon your-image.png` (1024x1024 PNG recommended) |
| App name / identifier | `src-tauri/tauri.conf.json` |
