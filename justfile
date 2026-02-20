# Wizard App — Tauri v2

export PATH := env('HOME') / ".cargo/bin:" + env('PATH')

bundle_dir := "src-tauri/target/release/bundle/macos"

wizard_name := "Wizard"
wizard_url := "http://wizard.local:9000/"
wizard_icon_color := env("DEFAULT_ICON_COLOR", "green")

# Development mode
dev:
    cd src-tauri && DEFAULT_ENDPOINT_URL="{{wizard_url}}" DEFAULT_ICON_COLOR="{{wizard_icon_color}}" cargo tauri dev

# Release build
build:
    cd src-tauri && DEFAULT_ENDPOINT_URL="{{wizard_url}}" DEFAULT_ICON_COLOR="{{wizard_icon_color}}" cargo tauri build --bundles app

# Build and install Wizard.app to /Applications
install: build
    @echo "Installing {{wizard_name}}.app to /Applications..."
    rm -rf "/Applications/{{wizard_name}}.app"
    cp -r "{{bundle_dir}}/{{wizard_name}}.app" /Applications/
    @echo "Installed to /Applications/{{wizard_name}}.app"

# Remove Wizard.app from /Applications
uninstall:
    rm -rf "/Applications/{{wizard_name}}.app"
    @echo "Removed /Applications/{{wizard_name}}.app"

# Generate ROYGBIV icon color variants from base icon
generate-icons:
    python3 scripts/generate_icons.py
