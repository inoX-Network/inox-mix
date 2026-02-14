// Modul: config — Konfigurationsverwaltung (SQLite + Presets)

pub mod presets;
pub mod database;
pub mod migration;

use serde::{Deserialize, Serialize};

/// Anwendungs-Konfiguration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Sample-Rate in Hz (Standard: 48000)
    pub sample_rate: u32,
    /// Buffer-Größe in Samples (Standard: 256)
    pub buffer_size: u32,
    /// Aktives Farbschema
    pub theme: String,
    /// Sprache der UI
    pub language: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000,
            buffer_size: 256,
            theme: "dark".to_string(),
            language: "de".to_string(),
        }
    }
}

/// Konfiguration aus der Datenbank lesen
pub fn get_config(_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: SQLite Abfrage
    todo!("get_config")
}

/// Konfiguration in die Datenbank schreiben
pub fn set_config(_key: &str, _value: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: SQLite Update/Insert
    todo!("set_config")
}
