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

use log::info;

/// Tauri-Anwendung starten, PipeWire verbinden, Datenbank initialisieren
fn main() {
    env_logger::init();
    info!("inoX-MIX v0.3 startet...");

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|_app| {
            // TODO: PipeWire-Session initialisieren
            // TODO: SQLite-Datenbank öffnen/erstellen
            // TODO: Standard-Config laden
            info!("Setup abgeschlossen");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // TODO: Tauri Commands registrieren
            // api::routes::get_system_info,
            // api::routes::get_config,
            // api::routes::set_config,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der Tauri-Anwendung");
}
