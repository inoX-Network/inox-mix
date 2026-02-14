// Modul: streamer/soundboard — Soundboard (Sound-Pads für Stream)
use serde::{Deserialize, Serialize};

/// Ein einzelnes Sound-Pad im Soundboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundPad {
    /// Eindeutige Pad-ID
    pub id: String,
    /// Anzeige-Name
    pub name: String,
    /// Pfad zur Audio-Datei
    pub file_path: String,
    /// Lautstärke in dB
    pub volume_db: f32,
    /// Tastenkürzel (optional)
    pub hotkey: Option<String>,
}

/// Soundboard verwaltet und spielt Sound-Pads ab
#[derive(Debug)]
pub struct Soundboard {
    /// Alle geladenen Pads
    pub pads: Vec<SoundPad>,
    /// Globale Lautstärke in dB
    pub master_volume_db: f32,
}

impl Soundboard {
    /// Neues Soundboard erstellen
    pub fn new() -> Self {
        // TODO: Leeres Soundboard initialisieren
        todo!("Soundboard::new")
    }

    /// Sound-Pad abspielen
    pub fn play(&self, _pad_id: &str) -> Result<(), String> {
        // TODO: Audio-Datei über rodio abspielen
        todo!("Soundboard::play")
    }

    /// Pad hinzufügen
    pub fn add_pad(&mut self, _pad: SoundPad) -> Result<(), String> {
        // TODO: Pad zur Liste hinzufügen
        todo!("Soundboard::add_pad")
    }
}
