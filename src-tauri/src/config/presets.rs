// Modul: config/presets — Preset- und Szenen-Verwaltung
use serde::{Deserialize, Serialize};

/// Ein gespeichertes Preset (Mixer-Snapshot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// Eindeutige Preset-ID
    pub id: String,
    /// Anzeige-Name
    pub name: String,
    /// Kategorie (z.B. "Streaming", "Gaming", "Podcast")
    pub category: String,
    /// Serialisierter Mixer-State als JSON
    pub state_json: String,
    /// Erstellungszeitpunkt (Unix Timestamp)
    pub created_at: i64,
}

/// Preset-Manager für Laden/Speichern/Wechseln
#[derive(Debug)]
pub struct PresetManager {
    // TODO: Aktives Preset
    // TODO: Datenbank-Referenz
}

impl PresetManager {
    /// Neuen Preset-Manager erstellen
    pub fn new() -> Self {
        // TODO: DB-Verbindung + Standard-Presets laden
        todo!("PresetManager::new")
    }

    /// Preset speichern
    pub fn save(&self, _preset: &Preset) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: In SQLite speichern
        todo!("PresetManager::save")
    }

    /// Preset laden und anwenden
    pub fn load(&self, _preset_id: &str) -> Result<Preset, Box<dyn std::error::Error>> {
        // TODO: Aus SQLite laden
        todo!("PresetManager::load")
    }

    /// Alle Presets auflisten
    pub fn list(&self) -> Result<Vec<Preset>, Box<dyn std::error::Error>> {
        // TODO: Alle aus DB laden
        todo!("PresetManager::list")
    }
}
