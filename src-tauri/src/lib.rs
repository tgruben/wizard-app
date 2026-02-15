use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{Manager, WebviewUrl};

const RELOAD_MENU_ID: &str = "reload_page";
const RELOAD_JS: &str = "window.location.reload()";

fn should_handle_reload(event_id: &str) -> bool {
    event_id == RELOAD_MENU_ID
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
            // Create a window pointing at the external URL
            let url = WebviewUrl::External("http://wizard.local:9000/".parse().unwrap());
            tauri::WebviewWindowBuilder::new(app, "main", url)
                .title("Wizard")
                .inner_size(1280.0, 800.0)
                .build()?;
            Ok(())
        })
        .on_menu_event(|app, event| {
            if should_handle_reload(event.id().as_ref()) {
                if let Some(webview) = app.get_webview_window("main") {
                    let _ = webview.eval(RELOAD_JS);
                }
            }
        })
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
