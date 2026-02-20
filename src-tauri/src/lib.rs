mod commands;
mod icon;
mod settings;

use std::process::Command;
use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::webview::NewWindowResponse;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

const RELOAD_MENU_ID: &str = "reload_page";
const RELOAD_JS: &str = "window.location.reload()";

fn should_handle_reload(id: &str) -> bool {
    id == RELOAD_MENU_ID
}

fn open_settings_window(app: &tauri::AppHandle) {
    // If settings window already exists, just focus it
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.set_focus();
        return;
    }

    let url = WebviewUrl::App("settings.html".into());
    let _ = WebviewWindowBuilder::new(app, "settings", url)
        .title("Settings")
        .inner_size(400.0, 280.0)
        .resizable(false)
        .build();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(|app| {
            let settings_item = MenuItemBuilder::new("Settings...")
                .id("settings")
                .accelerator("CmdOrCtrl+,")
                .build(app)?;

            let app_submenu = SubmenuBuilder::new(app, "Wizard")
                .about(None)
                .separator()
                .item(&settings_item)
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .show_all()
                .separator()
                .quit()
                .build()?;

            let edit_submenu = SubmenuBuilder::new(app, "Edit")
                .undo()
                .redo()
                .separator()
                .cut()
                .copy()
                .paste()
                .select_all()
                .build()?;

            let reload_item = MenuItemBuilder::new("Reload Page")
                .id(RELOAD_MENU_ID)
                .accelerator("CmdOrCtrl+R")
                .build(app)?;

            let view_submenu = SubmenuBuilder::new(app, "View")
                .item(&reload_item)
                .separator()
                .fullscreen()
                .build()?;

            let window_submenu = SubmenuBuilder::new(app, "Window")
                .minimize()
                .maximize()
                .separator()
                .close_window()
                .build()?;

            MenuBuilder::new(app)
                .items(&[&app_submenu, &edit_submenu, &view_submenu, &window_submenu])
                .build()
        })
        .setup(|app| {
            let handle = app.handle();
            let app_name = handle
                .config()
                .product_name
                .clone()
                .unwrap_or_else(|| "Wizard".to_string());

            // Load persisted settings
            let loaded = settings::load_settings(handle);
            let endpoint_url = loaded.endpoint_url;

            // Apply persisted dock icon color
            icon::set_dock_icon(handle, &loaded.icon_color);
            let url = WebviewUrl::External(endpoint_url.parse().unwrap());
            WebviewWindowBuilder::new(app, "main", url)
                .title(&app_name)
                .inner_size(1280.0, 800.0)
                .on_new_window(|url, _features| {
                    let _ = Command::new("open").arg(url.as_str()).spawn();
                    NewWindowResponse::Deny
                })
                .build()?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            let id = event.id().as_ref();
            if id == "settings" {
                open_settings_window(app);
            } else if should_handle_reload(id) {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval(RELOAD_JS);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_endpoint_url,
            commands::set_endpoint_url,
            commands::get_icon_color,
            commands::set_icon_color,
            commands::close_settings_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reload_menu_id_is_correct() {
        assert_eq!(RELOAD_MENU_ID, "reload_page");
    }

    #[test]
    fn reload_js_calls_location_reload() {
        assert_eq!(RELOAD_JS, "window.location.reload()");
    }

    #[test]
    fn should_handle_reload_returns_true_for_reload_id() {
        assert!(should_handle_reload(RELOAD_MENU_ID));
    }

    #[test]
    fn should_handle_reload_returns_false_for_other_ids() {
        assert!(!should_handle_reload("quit"));
        assert!(!should_handle_reload(""));
        assert!(!should_handle_reload("reload"));
    }
}
