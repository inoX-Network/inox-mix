// Modul: config/presets — Preset- und Szenen-Verwaltung
//
// Verwaltet Presets (vordefinierte Configs) und Scenes (User-Snapshots)
// SPEC: 10-presets-scenes

use serde::{Deserialize, Serialize};
use super::database::Database;
use std::sync::Arc;
use rusqlite::params;

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

/// Eine gespeicherte Scene (Kompletter Mixer-State Snapshot)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Eindeutige Scene-ID
    pub id: String,
    /// Anzeige-Name
    pub name: String,
    /// Kompletter Mixer-State als JSON (Strips, Buses, Routing, FX, Master)
    pub state_json: String,
    /// Erstellungszeitpunkt (Unix Timestamp)
    pub created_at: i64,
}

/// Scene-Liste-Eintrag (ohne state_json für Übersicht)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneInfo {
    pub id: String,
    pub name: String,
    pub created_at: i64,
}

/// Scene-Manager für Speichern/Laden/Löschen von Szenen
pub struct SceneManager {
    db: Arc<Database>,
}

impl SceneManager {
    /// Neuen Scene-Manager mit Datenbank-Referenz erstellen
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Scene speichern
    pub fn save_scene(&self, name: &str, state_json: &str) -> Result<String, String> {
        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        let id = format!("scene_{}", uuid::Uuid::new_v4());
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Zeit-Fehler: {}", e))?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO scenes (id, name, state_json, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![id, name, state_json, created_at],
        ).map_err(|e| format!("DB-Insert-Fehler: {}", e))?;

        log::info!("Scene gespeichert: {} (ID: {})", name, id);
        Ok(id)
    }

    /// Scene laden
    pub fn load_scene(&self, id: &str) -> Result<Scene, String> {
        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, state_json, created_at FROM scenes WHERE id = ?1"
        ).map_err(|e| format!("DB-Prepare-Fehler: {}", e))?;

        let scene = stmt.query_row(params![id], |row| {
            Ok(Scene {
                id: row.get(0)?,
                name: row.get(1)?,
                state_json: row.get(2)?,
                created_at: row.get(3)?,
            })
        }).map_err(|e| format!("Scene nicht gefunden: {}", e))?;

        log::info!("Scene geladen: {} (ID: {})", scene.name, scene.id);
        Ok(scene)
    }

    /// Scene löschen
    pub fn delete_scene(&self, id: &str) -> Result<(), String> {
        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        let rows_affected = conn.execute(
            "DELETE FROM scenes WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("DB-Delete-Fehler: {}", e))?;

        if rows_affected == 0 {
            return Err(format!("Scene nicht gefunden: {}", id));
        }

        log::info!("Scene gelöscht: {}", id);
        Ok(())
    }

    /// Alle Scenes auflisten (ohne state_json)
    pub fn list_scenes(&self) -> Result<Vec<SceneInfo>, String> {
        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, created_at FROM scenes ORDER BY created_at DESC"
        ).map_err(|e| format!("DB-Prepare-Fehler: {}", e))?;

        let scenes = stmt.query_map([], |row| {
            Ok(SceneInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        }).map_err(|e| format!("DB-Query-Fehler: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("DB-Collect-Fehler: {}", e))?;

        Ok(scenes)
    }
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
