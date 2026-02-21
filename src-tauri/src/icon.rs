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

/// Map a ROYGBIV color name to (R, G, B) in 0.0–1.0 range.
/// Returns None for unknown color names.
pub fn color_rgb(color: &str) -> Option<(f64, f64, f64)> {
    match color {
        "red"    => Some((0.898, 0.243, 0.243)),  // #e53e3e
        "orange" => Some((0.929, 0.537, 0.212)),  // #ed8936
        "yellow" => Some((0.925, 0.788, 0.294)),  // #ecc94b
        "green"  => Some((0.220, 0.631, 0.412)),  // #38a169
        "blue"   => Some((0.192, 0.510, 0.808)),  // #3182ce
        "indigo" => Some((0.353, 0.404, 0.847)),  // #5a67d8
        "violet" => Some((0.624, 0.478, 0.918)),  // #9f7aea
        _        => None,
    }
}

/// Set the title bar color of the given window to match a ROYGBIV color name.
/// On macOS, uses NSWindow's setTitlebarAppearsTransparent + setBackgroundColor.
/// No-op on other platforms.
#[cfg(target_os = "macos")]
pub fn set_titlebar_color(window: &tauri::WebviewWindow, color: &str) {
    let Some((r, g, b)) = color_rgb(color) else {
        eprintln!("Unknown titlebar color: {color}");
        return;
    };

    let _ = window.with_webview(move |webview| {
        unsafe {
            use objc2_app_kit::{NSColor, NSWindow};

            let ns_window: &NSWindow = &*webview.ns_window().cast();
            ns_window.setTitlebarAppearsTransparent(true);

            let bg = NSColor::colorWithSRGBRed_green_blue_alpha(r, g, b, 1.0);
            ns_window.setBackgroundColor(Some(&bg));
        }
    });
}

#[cfg(not(target_os = "macos"))]
pub fn set_titlebar_color(_window: &tauri::WebviewWindow, _color: &str) {
    // No-op on non-macOS platforms
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
