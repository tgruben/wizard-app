# Wizard App

A minimal Tauri v2 app that wraps `http://wizard.local:9000/` as a standalone macOS app.

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Tauri CLI
cargo install tauri-cli --version "^2"
```

## Dev mode

```bash
cd src-tauri
cargo tauri dev
```

## Build release .app

```bash
cd src-tauri
cargo tauri build
```

The `.app` bundle will be in `src-tauri/target/release/bundle/macos/`.

## Customization

- **Window size**: Edit `inner_size()` in `src/lib.rs`
- **App icon**: Replace files in `src-tauri/icons/` (use `cargo tauri icon your-image.png` to generate all sizes)
- **URL**: Change the URL string in `src/lib.rs`
