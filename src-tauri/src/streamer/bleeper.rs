// Modul: streamer/bleeper — Profanity Bleeper (Schimpfwort-Zensur via STT)
use serde::{Deserialize, Serialize};

/// Bleeper-Modus: Wie wird zensiert?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BleepMode {
    /// Klassischer Piep-Ton
    Beep,
    /// Stille
    Mute,
    /// Rauschen
    Noise,
    /// Rückwärts-Effekt
    Reverse,
    /// Benutzerdefinierter Sound
    Custom,
}

/// Profanity Bleeper Engine
#[derive(Debug)]
pub struct BleeperEngine {
    /// Aktiver Bleep-Modus
    pub mode: BleepMode,
    /// Piep-Frequenz in Hz (bei Beep-Modus)
    pub tone_hz: f32,
    /// Bleep-Lautstärke in dB
    pub volume_db: f32,
    /// Bleeper aktiviert
    pub armed: bool,
}

impl BleeperEngine {
    /// Neuen Bleeper erstellen
    pub fn new() -> Self {
        // TODO: Standard-Einstellungen
        todo!("BleeperEngine::new")
    }

    /// Wort zensieren (aufgerufen wenn STT ein Schimpfwort erkennt)
    pub fn censor(&mut self, _samples: &mut [f32], _start: usize, _end: usize) {
        // TODO: Bleep-Sound auf die erkannte Position anwenden
        todo!("BleeperEngine::censor")
    }
}
