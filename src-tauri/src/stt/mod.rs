// Modul: stt — Sprache-zu-Text Manager (VOSK Live + Whisper Genau)

pub mod vosk;
pub mod whisper;

use serde::{Deserialize, Serialize};

/// Erkanntes Wort mit Zeitstempel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognizedWord {
    /// Erkanntes Wort
    pub text: String,
    /// Start-Zeit in Sekunden
    pub start: f64,
    /// End-Zeit in Sekunden
    pub end: f64,
    /// Konfidenz (0.0 - 1.0)
    pub confidence: f32,
}

/// STT-Manager wählt zwischen VOSK (Live) und Whisper (Genau)
#[derive(Debug)]
pub struct SttManager {
    // TODO: VOSK-Instanz
    // TODO: Whisper-Instanz
    // TODO: Aktiver Modus
}

impl SttManager {
    /// Neuen STT-Manager erstellen
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Modelle laden
        todo!("SttManager::new")
    }
}
