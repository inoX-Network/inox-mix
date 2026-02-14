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

use audio::bus::{BusManager, OutputBus};
use audio::mixer::{InputStrip, MixerState};
use audio::routing::{RoutingManager, RoutingEntry};
use audio::pipewire as pw;
use config::database::Database;
use config::ConfigManager;
use fx::{FxChain, FxModuleInfo, FxModuleType};
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
    /// Bus-Manager mit allen Output-Bussen
    buses: Mutex<BusManager>,
    /// FX-Chain (Phase 1: Global, später pro Strip)
    fx_chain: Mutex<FxChain>,
    /// Routing-Manager (Audio-Routing Matrix)
    routing: Mutex<RoutingManager>,
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

// --- Bus Commands (Modul 04) ---

/// Alle Output-Busse als sortierte Liste abrufen
#[tauri::command]
fn get_buses(state: tauri::State<'_, AppState>) -> Result<Vec<OutputBus>, String> {
    let buses = state.buses.lock()
        .map_err(|e| format!("Bus-Lock-Fehler: {}", e))?;
    Ok(buses.get_buses())
}

/// Lautstärke eines Bus setzen (in dB)
#[tauri::command]
fn set_bus_volume(bus_id: String, volume_db: f32, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut buses = state.buses.lock()
        .map_err(|e| format!("Bus-Lock-Fehler: {}", e))?;
    buses.set_volume(&bus_id, volume_db)
}

/// Stummschaltung eines Bus setzen
#[tauri::command]
fn set_bus_mute(bus_id: String, muted: bool, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut buses = state.buses.lock()
        .map_err(|e| format!("Bus-Lock-Fehler: {}", e))?;
    buses.set_mute(&bus_id, muted)
}

// --- FX Commands (Modul 03 - Phase 1) ---

/// FX-Chain Module abrufen (Phase 1: Global, später pro strip_id)
#[tauri::command]
fn get_fx_chain(state: tauri::State<'_, AppState>) -> Result<Vec<FxModuleInfo>, String> {
    let fx = state.fx_chain.lock()
        .map_err(|e| format!("FX-Lock-Fehler: {}", e))?;
    Ok(fx.get_all_modules())
}

/// FX-Parameter setzen
#[tauri::command]
fn set_fx_param(
    module_type: FxModuleType,
    param_name: String,
    value: f32,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut fx = state.fx_chain.lock()
        .map_err(|e| format!("FX-Lock-Fehler: {}", e))?;
    fx.set_param(module_type, &param_name, value)
}

/// FX-Bypass setzen
#[tauri::command]
fn set_fx_bypass(
    module_type: FxModuleType,
    bypass: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut fx = state.fx_chain.lock()
        .map_err(|e| format!("FX-Lock-Fehler: {}", e))?;
    fx.set_bypass(module_type, bypass)
}

// --- Routing Commands (Modul 06) ---

/// Routing-Matrix abrufen
#[tauri::command]
fn get_routing_matrix(state: tauri::State<'_, AppState>) -> Result<Vec<RoutingEntry>, String> {
    let routing = state.routing.lock()
        .map_err(|e| format!("Routing-Lock-Fehler: {}", e))?;
    Ok(routing.get_routing_matrix())
}

/// Routing setzen (Source → Bus Verbindung)
#[tauri::command]
fn set_routing(
    source_id: String,
    bus_id: String,
    active: bool,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut routing = state.routing.lock()
        .map_err(|e| format!("Routing-Lock-Fehler: {}", e))?;
    routing.set_routing(&source_id, &bus_id, active)
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

            // 5. Bus-Manager erstellen
            let buses = BusManager::new();
            info!("Bus-Manager initialisiert mit {} Bussen", buses.bus_count());

            // 6. FX-Chain erstellen (Phase 1: Global)
            let fx_chain = FxChain::new();
            info!("FX-Chain initialisiert (Phase 1: HPF + Gate)");

            // 7. Routing-Manager erstellen
            let routing = RoutingManager::new();
            info!("Routing-Manager initialisiert");

            // 8. App-State registrieren
            app.manage(AppState {
                config_manager,
                mixer: Mutex::new(mixer),
                buses: Mutex::new(buses),
                fx_chain: Mutex::new(fx_chain),
                routing: Mutex::new(routing),
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
            get_buses,
            set_bus_volume,
            set_bus_mute,
            get_fx_chain,
            set_fx_param,
            set_fx_bypass,
            get_routing_matrix,
            set_routing,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der Tauri-Anwendung");
}
