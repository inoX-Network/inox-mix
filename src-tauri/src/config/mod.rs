// Modul: config — Konfigurationsverwaltung (SQLite + Presets)

pub mod database;
pub mod migration;
pub mod presets;

use database::Database;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Standard Sample-Rate in Hz
const DEFAULT_SAMPLE_RATE: u32 = 48000;
/// Standard Buffer-Größe in Samples
const DEFAULT_BUFFER_SIZE: u32 = 256;

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
            sample_rate: DEFAULT_SAMPLE_RATE,
            buffer_size: DEFAULT_BUFFER_SIZE,
            theme: "dark".to_string(),
            language: "de".to_string(),
        }
    }
}

/// Config-Manager verwaltet die Anwendungs-Konfiguration über SQLite
pub struct ConfigManager {
    /// Datenbank-Referenz (Thread-sicher)
    db: Arc<Database>,
}

impl ConfigManager {
    /// Neuen Config-Manager mit Datenbank-Referenz erstellen
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Standard-Config-Werte in die Datenbank schreiben (nur wenn noch nicht vorhanden)
    pub fn init_defaults(&self) -> Result<(), Box<dyn std::error::Error>> {
        let defaults = AppConfig::default();

        // Nur setzen wenn noch nicht vorhanden
        self.set_if_empty("audio.sample_rate", &defaults.sample_rate.to_string())?;
        self.set_if_empty("audio.buffer_size", &defaults.buffer_size.to_string())?;
        self.set_if_empty("ui.theme", &defaults.theme)?;
        self.set_if_empty("ui.language", &defaults.language)?;

        info!("Standard-Konfiguration initialisiert");
        Ok(())
    }

    /// Config-Wert lesen — gibt Default zurück wenn nicht gefunden
    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        self.db.get(key)
    }

    /// Config-Wert setzen
    pub fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.db.set(key, value)
    }

    /// Config-Wert nur setzen wenn noch nicht vorhanden
    fn set_if_empty(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.db.get(key)?.is_none() {
            self.db.set(key, value)?;
        }
        Ok(())
    }

    /// Komplette AppConfig aus der Datenbank laden
    pub fn load_app_config(&self) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let defaults = AppConfig::default();

        let sample_rate = self
            .get("audio.sample_rate")?
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(defaults.sample_rate);

        let buffer_size = self
            .get("audio.buffer_size")?
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(defaults.buffer_size);

        let theme = self.get("ui.theme")?.unwrap_or(defaults.theme);

        let language = self.get("ui.language")?.unwrap_or(defaults.language);

        Ok(AppConfig {
            sample_rate,
            buffer_size,
            theme,
            language,
        })
    }

    /// AppConfig komplett in die Datenbank speichern
    pub fn save_app_config(&self, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.set("audio.sample_rate", &config.sample_rate.to_string())?;
        self.set("audio.buffer_size", &config.buffer_size.to_string())?;
        self.set("ui.theme", &config.theme)?;
        self.set("ui.language", &config.language)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> ConfigManager {
        let db = Arc::new(Database::open_in_memory().unwrap());
        ConfigManager::new(db)
    }

    #[test]
    fn test_init_defaults() {
        let cm = setup();
        let result = cm.init_defaults();
        assert!(result.is_ok());

        let sr = cm.get("audio.sample_rate").unwrap();
        assert_eq!(sr, Some("48000".to_string()));
    }

    #[test]
    fn test_init_defaults_no_overwrite() {
        let cm = setup();

        // Eigenen Wert setzen
        cm.set("audio.sample_rate", "44100").unwrap();

        // Defaults initialisieren — sollte nicht überschreiben
        cm.init_defaults().unwrap();

        let sr = cm.get("audio.sample_rate").unwrap();
        assert_eq!(sr, Some("44100".to_string()));
    }

    #[test]
    fn test_load_app_config_defaults() {
        let cm = setup();
        cm.init_defaults().unwrap();

        let config = cm.load_app_config().unwrap();
        assert_eq!(config.sample_rate, 48000);
        assert_eq!(config.buffer_size, 256);
        assert_eq!(config.theme, "dark");
        assert_eq!(config.language, "de");
    }

    #[test]
    fn test_save_and_load_app_config() {
        let cm = setup();

        let config = AppConfig {
            sample_rate: 44100,
            buffer_size: 512,
            theme: "light".to_string(),
            language: "en".to_string(),
        };

        cm.save_app_config(&config).unwrap();
        let loaded = cm.load_app_config().unwrap();

        assert_eq!(loaded.sample_rate, 44100);
        assert_eq!(loaded.buffer_size, 512);
        assert_eq!(loaded.theme, "light");
        assert_eq!(loaded.language, "en");
    }

    #[test]
    fn test_get_set() {
        let cm = setup();

        cm.set("test.key", "hello").unwrap();
        let value = cm.get("test.key").unwrap();
        assert_eq!(value, Some("hello".to_string()));
    }

    #[test]
    fn test_get_nonexistent() {
        let cm = setup();

        let value = cm.get("nonexistent").unwrap();
        assert_eq!(value, None);
    }
}
