// Modul: recording — Audio-Aufnahme Engine

pub mod encoder;

use serde::{Deserialize, Serialize};

/// Aufnahme-Format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordingFormat {
    /// WAV (unkomprimiert, höchste Qualität)
    Wav,
    /// FLAC (verlustfrei komprimiert)
    Flac,
    /// OGG Vorbis (verlustbehaftet)
    Ogg,
}

/// Aufnahme-Engine verwaltet aktive Aufnahmen
#[derive(Debug)]
pub struct RecordingEngine {
    // TODO: Aktive Aufnahmen
    // TODO: Ausgabe-Pfad
    // TODO: Format-Konfiguration
}

impl RecordingEngine {
    /// Neue Recording-Engine erstellen
    pub fn new() -> Self {
        // TODO: Standard-Pfad und Format setzen
        todo!("RecordingEngine::new")
    }

    /// Aufnahme starten
    pub fn start(&mut self, _channel_id: &str, _format: RecordingFormat) -> Result<(), String> {
        // TODO: Encoder starten, Audio-Stream anzapfen
        todo!("RecordingEngine::start")
    }

    /// Aufnahme stoppen
    pub fn stop(&mut self, _channel_id: &str) -> Result<String, String> {
        // TODO: Encoder finalisieren, Dateipfad zurückgeben
        todo!("RecordingEngine::stop")
    }
}
