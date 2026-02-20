use crate::icon;
use crate::settings;
use tauri::{Manager, Url};

#[tauri::command]
pub fn get_endpoint_url(app: tauri::AppHandle) -> String {
    settings::load_settings(&app).endpoint_url
}

#[tauri::command]
pub fn set_endpoint_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    // Validate the URL
    let parsed = Url::parse(&url).map_err(|e| format!("Invalid URL: {e}"))?;
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err("URL must use http or https".to_string());
    }

    // Load existing settings to preserve other fields
    let mut current = settings::load_settings(&app);
    current.endpoint_url = url;
    settings::save_settings(&app, &current)?;

    // Navigate the main window to the new URL
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.navigate(parsed).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn get_icon_color(app: tauri::AppHandle) -> String {
    settings::load_settings(&app).icon_color
}

#[tauri::command]
pub fn set_icon_color(app: tauri::AppHandle, color: String) -> Result<(), String> {
    settings::validate_color(&color)?;

    // Load existing settings to preserve other fields
    let mut current = settings::load_settings(&app);
    current.icon_color = color.clone();
    settings::save_settings(&app, &current)?;

    // Update dock icon immediately
    icon::set_dock_icon(&app, &color);

    Ok(())
}

#[tauri::command]
pub fn close_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}
