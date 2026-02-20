use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

const DEFAULT_ENDPOINT_URL: &str = match option_env!("DEFAULT_ENDPOINT_URL") {
    Some(url) => url,
    None => "http://wizard.local:9000/",
};
const SETTINGS_FILE: &str = "settings.json";

pub const DEFAULT_ICON_COLOR: &str = match option_env!("DEFAULT_ICON_COLOR") {
    Some(color) => color,
    None => "green",
};

pub const VALID_COLORS: &[&str] = &[
    "red", "orange", "yellow", "green", "blue", "indigo", "violet",
];

pub fn validate_color(color: &str) -> Result<(), String> {
    if VALID_COLORS.contains(&color) {
        Ok(())
    } else {
        Err(format!(
            "Invalid color '{}'. Valid colors: {}",
            color,
            VALID_COLORS.join(", ")
        ))
    }
}

fn default_icon_color() -> String {
    DEFAULT_ICON_COLOR.to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub endpoint_url: String,
    #[serde(default = "default_icon_color")]
    pub icon_color: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            endpoint_url: DEFAULT_ENDPOINT_URL.to_string(),
            icon_color: default_icon_color(),
        }
    }
}

fn settings_path(app: &tauri::AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap().join(SETTINGS_FILE)
}

pub fn load_settings(app: &tauri::AppHandle) -> Settings {
    let path = settings_path(app);
    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

pub fn save_settings(app: &tauri::AppHandle, settings: &Settings) -> Result<(), String> {
    let path = settings_path(app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}
