# Wizard App — Tauri v2

app_name := "Wizard"
bundle_dir := "src-tauri/target/release/bundle/macos"

# Development mode
dev:
    cd src-tauri && cargo tauri dev

# Release build
build:
    cd src-tauri && cargo tauri build

# Build and copy Wizard.app to /Applications
install: build
    @echo "Installing {{app_name}}.app to /Applications..."
    rm -rf "/Applications/{{app_name}}.app"
    cp -r "{{bundle_dir}}/{{app_name}}.app" /Applications/
    @echo "Installed to /Applications/{{app_name}}.app"

# Remove Wizard.app from /Applications
uninstall:
    rm -rf "/Applications/{{app_name}}.app"
    @echo "Removed /Applications/{{app_name}}.app"
