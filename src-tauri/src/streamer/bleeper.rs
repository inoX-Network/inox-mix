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
        Self {
            mode: BleepMode::Beep,
            tone_hz: 1000.0, // Standard: 1000Hz Piep
            volume_db: -6.0, // -6dB Standard-Lautstärke
            armed: false,
        }
    }

    /// Bleeper-Parameter setzen
    pub fn set_mode(&mut self, mode: BleepMode) {
        self.mode = mode;
    }

    pub fn set_tone(&mut self, tone_hz: f32) {
        self.tone_hz = tone_hz.clamp(200.0, 2000.0);
    }

    pub fn set_volume(&mut self, volume_db: f32) {
        self.volume_db = volume_db.clamp(-30.0, 0.0);
    }

    pub fn set_armed(&mut self, armed: bool) {
        self.armed = armed;
    }

    /// Wort zensieren (aufgerufen wenn STT ein Schimpfwort erkennt)
    pub fn censor(&mut self, _samples: &mut [f32], _start: usize, _end: usize) {
        if !self.armed {
            return;
        }

        // TODO Phase 2: Echte Bleep-Implementierung
        // - Beep: Sinus-Generator mit self.tone_hz und self.volume_db
        // - Mute: Audio nullen
        // - Noise: Weißes Rauschen generieren
        // - Reverse: Audio rückwärts abspielen
        // - Custom: Custom-Sample laden und einfügen
        //
        // Phase 1: Stub-Implementierung (kein Effekt)
    }
}
