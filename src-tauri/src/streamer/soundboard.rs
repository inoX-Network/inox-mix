// Modul: streamer/soundboard — Soundboard (Sound-Pads für Stream)
//
// Verwaltet Sound-Pads und spielt sie über rodio ab
// SPEC: 13-soundboard

use serde::{Deserialize, Serialize};
use crate::config::database::Database;
use std::sync::Arc;
use std::path::PathBuf;
use rusqlite::params;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

/// Ein einzelnes Sound-Pad im Soundboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEntry {
    /// Eindeutige Sound-ID
    pub id: String,
    /// Anzeige-Name
    pub name: String,
    /// Pfad zur Audio-Datei (WAV/MP3/OGG)
    pub file_path: String,
    /// Hotkey (optional, z.B. "F1", "Ctrl+1")
    pub hotkey: Option<String>,
    /// Bus-ID für Routing (Standard: "B1")
    pub bus_id: String,
    /// Lautstärke in dB
    pub volume_db: f32,
    /// Erstellungszeitpunkt
    pub created_at: i64,
}

/// Soundboard Manager für Sound-Playback und Verwaltung
pub struct SoundboardManager {
    db: Arc<Database>,
    /// Globale Soundboard-Lautstärke
    master_volume_db: f32,
}

impl SoundboardManager {
    /// Neuen Soundboard-Manager erstellen
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            master_volume_db: 0.0,
        }
    }

    /// Sound hinzufügen
    pub fn add_sound(
        &self,
        name: &str,
        file_path: &str,
        hotkey: Option<String>,
        bus_id: Option<String>,
    ) -> Result<String, String> {
        // Prüfen ob Datei existiert
        if !PathBuf::from(file_path).exists() {
            return Err(format!("Audio-Datei nicht gefunden: {}", file_path));
        }

        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        let id = format!("sound_{}", uuid::Uuid::new_v4());
        let bus_id = bus_id.unwrap_or_else(|| "B1".to_string());
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Zeit-Fehler: {}", e))?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO sounds (id, name, file_path, hotkey, bus_id, volume_db, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, name, file_path, hotkey, bus_id, 0.0, created_at],
        ).map_err(|e| format!("DB-Insert-Fehler: {}", e))?;

        log::info!("Sound hinzugefügt: {} → {}", name, file_path);
        Ok(id)
    }

    /// Sound abspielen
    pub fn play_sound(&self, sound_id: &str) -> Result<(), String> {
        // Sound aus DB laden
        let sound = self.get_sound(sound_id)?;

        // Audio-Datei öffnen
        let file = File::open(&sound.file_path)
            .map_err(|e| format!("Konnte Audio-Datei nicht öffnen: {}", e))?;

        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Audio-Decoder-Fehler: {}", e))?;

        // Sink erstellen und abspielen (Fire-and-Forget)
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Audio-Output-Fehler: {}", e))?;

        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("Sink-Fehler: {}", e))?;

        // Volume setzen (dB → linear)
        let total_volume_db = sound.volume_db + self.master_volume_db;
        let volume_linear = 10.0_f32.powf(total_volume_db / 20.0);
        sink.set_volume(volume_linear);

        sink.append(source);
        sink.detach(); // Fire-and-Forget: Sink spielt ab und wird automatisch gedropped

        log::info!("Sound wird abgespielt: {} ({})", sound.name, sound.file_path);
        Ok(())
    }

    /// Sound stoppen (TODO: benötigt Sink-Tracking)
    pub fn stop_sound(&self, _sound_id: &str) -> Result<(), String> {
        // TODO: Für Stop-Funktion müssten wir die Sinks tracken
        Err("Stop-Funktion noch nicht implementiert (benötigt Sink-Tracking)".to_string())
    }

    /// Sound löschen
    pub fn remove_sound(&self, sound_id: &str) -> Result<(), String> {
        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        let rows = conn.execute(
            "DELETE FROM sounds WHERE id = ?1",
            params![sound_id],
        ).map_err(|e| format!("DB-Delete-Fehler: {}", e))?;

        if rows == 0 {
            return Err(format!("Sound nicht gefunden: {}", sound_id));
        }

        log::info!("Sound gelöscht: {}", sound_id);
        Ok(())
    }

    /// Alle Sounds auflisten
    pub fn get_sounds(&self) -> Result<Vec<SoundEntry>, String> {
        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, file_path, hotkey, bus_id, volume_db, created_at
             FROM sounds ORDER BY created_at ASC"
        ).map_err(|e| format!("DB-Prepare-Fehler: {}", e))?;

        let sounds = stmt.query_map([], |row| {
            Ok(SoundEntry {
                id: row.get(0)?,
                name: row.get(1)?,
                file_path: row.get(2)?,
                hotkey: row.get(3)?,
                bus_id: row.get(4)?,
                volume_db: row.get(5)?,
                created_at: row.get(6)?,
            })
        }).map_err(|e| format!("DB-Query-Fehler: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("DB-Collect-Fehler: {}", e))?;

        Ok(sounds)
    }

    /// Einzelnen Sound laden
    fn get_sound(&self, sound_id: &str) -> Result<SoundEntry, String> {
        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        let mut stmt = conn.prepare(
            "SELECT id, name, file_path, hotkey, bus_id, volume_db, created_at
             FROM sounds WHERE id = ?1"
        ).map_err(|e| format!("DB-Prepare-Fehler: {}", e))?;

        stmt.query_row(params![sound_id], |row| {
            Ok(SoundEntry {
                id: row.get(0)?,
                name: row.get(1)?,
                file_path: row.get(2)?,
                hotkey: row.get(3)?,
                bus_id: row.get(4)?,
                volume_db: row.get(5)?,
                created_at: row.get(6)?,
            })
        }).map_err(|e| format!("Sound nicht gefunden: {}", e))
    }

    /// Sound-Volume setzen
    pub fn set_sound_volume(&self, sound_id: &str, volume_db: f32) -> Result<(), String> {
        let conn = self.db.conn.lock()
            .map_err(|e| format!("DB-Lock-Fehler: {}", e))?;

        conn.execute(
            "UPDATE sounds SET volume_db = ?1 WHERE id = ?2",
            params![volume_db, sound_id],
        ).map_err(|e| format!("DB-Update-Fehler: {}", e))?;

        Ok(())
    }

    /// Master-Volume setzen
    pub fn set_master_volume(&mut self, volume_db: f32) {
        self.master_volume_db = volume_db;
    }

    /// Master-Volume abrufen
    pub fn get_master_volume(&self) -> f32 {
        self.master_volume_db
    }
}
