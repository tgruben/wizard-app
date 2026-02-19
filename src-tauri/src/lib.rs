mod commands;
mod settings;

use std::process::Command;
use tauri::menu::{MenuItem, PredefinedMenuItem, Submenu};
use tauri::webview::NewWindowResponse;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

fn open_settings_window(app: &tauri::AppHandle) {
    // If settings window already exists, just focus it
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.set_focus();
        return;
    }

    let url = WebviewUrl::App("settings.html".into());
    let _ = WebviewWindowBuilder::new(app, "settings", url)
        .title("Settings")
        .inner_size(400.0, 160.0)
        .resizable(false)
        .build();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(|app| {
            let app_submenu = SubmenuBuilder::new(app, "Wizard")
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
                .close_window()
                .build()?;

            MenuBuilder::new(app)
                .items(&[&app_submenu, &edit_submenu, &view_submenu, &window_submenu])
                .build()
        })
        .setup(|app| {
            // Build menu bar
            let handle = app.handle();
            let app_name = handle
                .config()
                .product_name
                .clone()
                .unwrap_or_else(|| "Wizard".to_string());

            let settings_item =
                MenuItem::with_id(handle, "settings", "Settings...", true, Some("CmdOrCtrl+,"))?;

            let app_menu = Submenu::with_items(
                handle,
                &app_name,
                true,
                &[
                    &PredefinedMenuItem::about(handle, Some(&format!("About {app_name}")), None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &settings_item,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::services(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::hide(handle, None)?,
                    &PredefinedMenuItem::hide_others(handle, None)?,
                    &PredefinedMenuItem::show_all(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::quit(handle, None)?,
                ],
            )?;

            let edit_menu = Submenu::with_items(
                handle,
                "Edit",
                true,
                &[
                    &PredefinedMenuItem::undo(handle, None)?,
                    &PredefinedMenuItem::redo(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::cut(handle, None)?,
                    &PredefinedMenuItem::copy(handle, None)?,
                    &PredefinedMenuItem::paste(handle, None)?,
                    &PredefinedMenuItem::select_all(handle, None)?,
                ],
            )?;

            let view_menu = Submenu::with_items(
                handle,
                "View",
                true,
                &[&PredefinedMenuItem::fullscreen(handle, None)?],
            )?;

            let window_menu = Submenu::with_items(
                handle,
                "Window",
                true,
                &[
                    &PredefinedMenuItem::minimize(handle, None)?,
                    &PredefinedMenuItem::maximize(handle, None)?,
                    &PredefinedMenuItem::separator(handle)?,
                    &PredefinedMenuItem::close_window(handle, None)?,
                ],
            )?;

            let menu = tauri::menu::Menu::with_items(
                handle,
                &[&app_menu, &edit_menu, &view_menu, &window_menu],
            )?;
            app.set_menu(menu)?;

            // Load persisted URL and create main window
            let endpoint_url = settings::load_settings(handle).endpoint_url;
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
            if event.id().as_ref() == "settings" {
                open_settings_window(app);
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_endpoint_url,
            commands::set_endpoint_url,
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
