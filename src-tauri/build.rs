use std::fs;
use std::path::Path;

fn main() {
    // Allow build-time icon color selection via DEFAULT_ICON_COLOR env var.
    // Copies the selected color's icons to the default icons/ directory so
    // the bundled .app uses that color.
    println!("cargo:rerun-if-env-changed=DEFAULT_ICON_COLOR");

    let color = std::env::var("DEFAULT_ICON_COLOR").unwrap_or_else(|_| "green".to_string());
    let color_dir = Path::new("icons/colors").join(&color);

    if color_dir.exists() {
        let icon_files = &[
            "32x32.png",
            "128x128.png",
            "128x128@2x.png",
            "icon.icns",
            "icon.ico",
            "icon.png",
        ];

        for file in icon_files {
            let src = color_dir.join(file);
            let dst = Path::new("icons").join(file);
            if src.exists() {
                if let Err(e) = fs::copy(&src, &dst) {
                    println!("cargo:warning=Failed to copy {}: {}", src.display(), e);
                }
            }
        }
    }

    tauri_build::build()
}
