use std::path::PathBuf;
use tauri::Manager;

/// Resolve the bundled resource path to a color variant's icon.
pub fn icon_path_for_color(app: &tauri::AppHandle, color: &str) -> Option<PathBuf> {
    app.path()
        .resource_dir()
        .ok()
        .map(|dir| dir.join(format!("icons/colors/{color}/128x128@2x.png")))
        .filter(|p| p.exists())
}

/// Set the dock icon to the given color variant.
/// On macOS, uses NSApplication::setApplicationIconImage.
/// No-op on other platforms.
#[cfg(target_os = "macos")]
pub fn set_dock_icon(app: &tauri::AppHandle, color: &str) {
    use objc2::MainThreadMarker;
    use objc2::AnyThread;
    use objc2_app_kit::{NSApplication, NSImage};
    use objc2_foundation::NSString;

    let Some(icon_path) = icon_path_for_color(app, color) else {
        eprintln!("Icon not found for color: {color}");
        return;
    };

    unsafe {
        let path_str = NSString::from_str(icon_path.to_string_lossy().as_ref());
        let image = NSImage::initWithContentsOfFile(NSImage::alloc(), &path_str);
        if let Some(image) = image {
            // We know we're on the main thread (Tauri setup and IPC run on main thread)
            let mtm = MainThreadMarker::new_unchecked();
            let ns_app = NSApplication::sharedApplication(mtm);
            ns_app.setApplicationIconImage(Some(&image));
        } else {
            eprintln!("Failed to load icon image from: {}", icon_path.display());
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn set_dock_icon(_app: &tauri::AppHandle, _color: &str) {
    // No-op on non-macOS platforms
}
