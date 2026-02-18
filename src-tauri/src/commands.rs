use crate::settings::{self, Settings};
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

    // Save to disk
    let new_settings = Settings { endpoint_url: url };
    settings::save_settings(&app, &new_settings)?;

    // Navigate the main window to the new URL
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.navigate(parsed).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn close_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}
