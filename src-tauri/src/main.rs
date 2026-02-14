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

use audio::mixer::{InputStrip, MixerState};
use audio::pipewire as pw;
use config::database::Database;
use config::ConfigManager;
use log::{info, error, warn};
use std::sync::{Arc, Mutex};
use tauri::{Manager, Emitter};

/// Dateiname der SQLite-Datenbank
const DB_FILENAME: &str = "inox-mix.db";

/// Globaler App-State der über Tauri verwaltet wird
struct AppState {
    /// Config-Manager für Datenbank-Zugriff
    config_manager: ConfigManager,
    /// Mixer-State mit allen Input-Strips
    mixer: Mutex<MixerState>,
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

// --- Mixer Commands (Modul 02) ---

/// Alle Input-Strips als sortierte Liste abrufen
#[tauri::command]
fn get_strips(state: tauri::State<'_, AppState>) -> Result<Vec<InputStrip>, String> {
    let mixer = state.mixer.lock()
        .map_err(|e| format!("Mixer-Lock-Fehler: {}", e))?;
    Ok(mixer.get_strips())
}

/// Lautstärke eines Strips setzen (in dB)
#[tauri::command]
fn set_strip_volume(strip_id: String, volume_db: f32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut mixer = state.mixer.lock()
        .map_err(|e| format!("Mixer-Lock-Fehler: {}", e))?;
    mixer.set_volume(&strip_id, volume_db)
}

/// Gain eines Strips setzen (in dB)
#[tauri::command]
fn set_strip_gain(strip_id: String, gain_db: f32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut mixer = state.mixer.lock()
        .map_err(|e| format!("Mixer-Lock-Fehler: {}", e))?;
    mixer.set_gain(&strip_id, gain_db)
}

/// Stummschaltung eines Strips setzen
#[tauri::command]
fn set_strip_mute(strip_id: String, muted: bool, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut mixer = state.mixer.lock()
        .map_err(|e| format!("Mixer-Lock-Fehler: {}", e))?;
    mixer.set_mute(&strip_id, muted)
}

/// Solo-Modus eines Strips setzen
#[tauri::command]
fn set_strip_solo(strip_id: String, solo: bool, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut mixer = state.mixer.lock()
        .map_err(|e| format!("Mixer-Lock-Fehler: {}", e))?;
    mixer.set_solo(&strip_id, solo)
}

/// Bus-Routing eines Strips ändern (Bus hinzufügen/entfernen)
#[tauri::command]
fn set_strip_bus(strip_id: String, bus_id: String, active: bool, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut mixer = state.mixer.lock()
        .map_err(|e| format!("Mixer-Lock-Fehler: {}", e))?;
    mixer.set_bus_routing(&strip_id, &bus_id, active)
}

/// Neuen Virtual-Strip hinzufügen
#[tauri::command]
fn add_virtual_strip(state: tauri::State<'_, AppState>) -> Result<InputStrip, String> {
    let mut mixer = state.mixer.lock()
        .map_err(|e| format!("Mixer-Lock-Fehler: {}", e))?;
    mixer.add_virtual_strip()
}

/// Virtual-Strip entfernen
#[tauri::command]
fn remove_virtual_strip(strip_id: String, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut mixer = state.mixer.lock()
        .map_err(|e| format!("Mixer-Lock-Fehler: {}", e))?;
    mixer.remove_virtual_strip(&strip_id)
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

            // 4. Mixer-State erstellen
            let mixer = MixerState::new();
            info!("Mixer initialisiert mit {} Strips", mixer.strip_count());

            // 5. App-State registrieren
            app.manage(AppState {
                config_manager,
                mixer: Mutex::new(mixer),
            });

            info!("Setup abgeschlossen");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_config,
            set_config,
            get_strips,
            set_strip_volume,
            set_strip_gain,
            set_strip_mute,
            set_strip_solo,
            set_strip_bus,
            add_virtual_strip,
            remove_virtual_strip,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der Tauri-Anwendung");
}
