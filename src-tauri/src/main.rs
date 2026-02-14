// Modul: main — Tauri Entry-Point mit PipeWire- und Datenbank-Initialisierung
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod fx;
mod streamer;
mod stt;
mod config;
mod api;
mod recording;
mod calibrate;
mod updater;

use audio::pipewire as pw;
use config::database::Database;
use config::ConfigManager;
use log::{info, error, warn};
use std::sync::Arc;
use tauri::{Manager, Emitter};

/// Dateiname der SQLite-Datenbank
const DB_FILENAME: &str = "inox-mix.db";

/// Globaler App-State der über Tauri verwaltet wird
struct AppState {
    /// Config-Manager für Datenbank-Zugriff
    config_manager: ConfigManager,
}

// --- Tauri Commands ---

/// System-Informationen abrufen (PipeWire-Version, Sample-Rate, Buffer-Size)
#[tauri::command]
fn get_system_info() -> Result<serde_json::Value, String> {
    let pw_info = pw::get_pipewire_info();

    Ok(serde_json::json!({
        "app_version": env!("CARGO_PKG_VERSION"),
        "pipewire_version": pw_info.version,
        "pipewire_running": pw_info.running,
        "sample_rate": pw_info.sample_rate,
        "buffer_size": pw_info.buffer_size,
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
    }))
}

/// Config-Wert aus der Datenbank lesen
#[tauri::command]
fn get_config(key: String, state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    state.config_manager.get(&key)
        .map_err(|e| format!("Config-Fehler: {}", e))
}

/// Config-Wert in die Datenbank schreiben
#[tauri::command]
fn set_config(key: String, value: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.config_manager.set(&key, &value)
        .map_err(|e| format!("Config-Fehler: {}", e))
}

/// Datenbank-Pfad ermitteln (im Tauri App-Data Verzeichnis)
fn get_db_path(app: &tauri::App) -> Result<String, Box<dyn std::error::Error>> {
    let app_data = app.path().app_data_dir()
        .map_err(|e| format!("App-Data Verzeichnis nicht gefunden: {}", e))?;
    let db_path = app_data.join(DB_FILENAME);
    Ok(db_path.to_string_lossy().to_string())
}

/// Tauri-Anwendung starten, PipeWire verbinden, Datenbank initialisieren
fn main() {
    env_logger::init();
    info!("inoX-MIX v{} startet...", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // 1. SQLite-Datenbank initialisieren
            let db_path = get_db_path(app)?;
            info!("Datenbank-Pfad: {}", db_path);

            let db = Arc::new(Database::open(&db_path).map_err(|e| {
                error!("Datenbank konnte nicht geöffnet werden: {}", e);
                e
            })?);

            // 2. Config-Manager erstellen und Defaults initialisieren
            let config_manager = ConfigManager::new(Arc::clone(&db));
            config_manager.init_defaults().map_err(|e| {
                error!("Config-Defaults konnten nicht gesetzt werden: {}", e);
                e
            })?;

            info!("Datenbank und Config initialisiert");

            // 3. PipeWire-Verfügbarkeit prüfen
            match pw::check_pipewire_available() {
                Ok(()) => info!("PipeWire verfügbar"),
                Err(msg) => {
                    warn!("PipeWire-Warnung: {}", msg);
                    // Warnung ans Frontend senden statt abzubrechen
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.emit("pipewire-warning", msg);
                    }
                }
            }

            // 4. App-State registrieren
            app.manage(AppState { config_manager });

            info!("Setup abgeschlossen");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_config,
            set_config,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der Tauri-Anwendung");
}
