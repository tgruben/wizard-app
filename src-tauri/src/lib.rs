use tauri::WebviewUrl;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Create a window pointing at the external URL
            let url = WebviewUrl::External("http://wizard.local:9000/".parse().unwrap());
            tauri::WebviewWindowBuilder::new(app, "main", url)
                .title("Wizard")
                .inner_size(1280.0, 800.0)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
