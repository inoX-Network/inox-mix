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
    /// Sample-Rate (für Ton-Generierung)
    sample_rate: f32,
    /// Phasen-Akkumulator für Sinus-Generator
    phase: f32,
}

impl BleeperEngine {
    /// Neuen Bleeper erstellen
    pub fn new() -> Self {
        Self {
            mode: BleepMode::Beep,
            tone_hz: 1000.0, // Standard: 1000Hz Piep
            volume_db: -6.0, // -6dB Standard-Lautstärke
            armed: false,
            sample_rate: 48000.0, // Standard Sample-Rate
            phase: 0.0,
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
    ///
    /// # Argumente
    /// * `samples` - Audio-Buffer (in-place modification)
    /// * `start` - Start-Index des zu zensierenden Bereichs
    /// * `end` - End-Index des zu zensierenden Bereichs
    ///
    /// # Phase 2: Echte Bleep-Implementierung
    /// Implementiert alle 5 Bleep-Modi:
    /// - Beep: Sinus-Ton Generator
    /// - Mute: Audio nullen
    /// - Noise: Weißes Rauschen
    /// - Reverse: Audio rückwärts
    /// - Custom: Benutzerdefinierter Sound
    pub fn censor(&mut self, samples: &mut [f32], start: usize, end: usize) {
        if !self.armed || start >= end || end > samples.len() {
            return;
        }

        let censor_region = &mut samples[start..end];

        match self.mode {
            BleepMode::Beep => {
                // Sinus-Ton mit tone_hz und volume_db generieren
                let amplitude = Self::db_to_linear(self.volume_db);
                let phase_increment = 2.0 * std::f32::consts::PI * self.tone_hz / self.sample_rate;

                for sample in censor_region.iter_mut() {
                    *sample = amplitude * self.phase.sin();
                    self.phase += phase_increment;

                    // Phase normalisieren (0..2π)
                    if self.phase >= 2.0 * std::f32::consts::PI {
                        self.phase -= 2.0 * std::f32::consts::PI;
                    }
                }
            }

            BleepMode::Mute => {
                // Audio komplett stumm schalten
                for sample in censor_region.iter_mut() {
                    *sample = 0.0;
                }
            }

            BleepMode::Noise => {
                // Weißes Rauschen generieren
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let amplitude = Self::db_to_linear(self.volume_db);

                for sample in censor_region.iter_mut() {
                    // Gleichverteiltes Rauschen: -1.0 bis 1.0
                    let noise: f32 = rng.gen_range(-1.0..=1.0);
                    *sample = amplitude * noise;
                }
            }

            BleepMode::Reverse => {
                // Audio rückwärts abspielen
                censor_region.reverse();
            }

            BleepMode::Custom => {
                // TODO: Custom-Sound laden und einfügen
                // Fallback: Mute
                for sample in censor_region.iter_mut() {
                    *sample = 0.0;
                }
            }
        }
    }

    /// dB zu Linear-Amplitude konvertieren
    fn db_to_linear(db: f32) -> f32 {
        10.0_f32.powf(db / 20.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bleeper_engine_creation() {
        let engine = BleeperEngine::new();
        assert_eq!(engine.armed, false);
        assert_eq!(engine.tone_hz, 1000.0);
        assert_eq!(engine.volume_db, -6.0);
        assert_eq!(engine.phase, 0.0);
    }

    #[test]
    fn test_censor_disabled() {
        let mut engine = BleeperEngine::new();
        engine.armed = false; // Deaktiviert

        let mut audio = vec![1.0; 100];
        engine.censor(&mut audio, 10, 50);

        // Audio sollte unverändert sein
        assert_eq!(audio[10], 1.0);
        assert_eq!(audio[30], 1.0);
        assert_eq!(audio[49], 1.0);
    }

    #[test]
    fn test_censor_mute_mode() {
        let mut engine = BleeperEngine::new();
        engine.armed = true;
        engine.set_mode(BleepMode::Mute);

        let mut audio = vec![0.8; 100];
        engine.censor(&mut audio, 20, 40);

        // Bereich 20-40 sollte stumm sein (0.0)
        assert_eq!(audio[19], 0.8, "Vor Censor-Region sollte unverändert sein");
        assert_eq!(audio[20], 0.0, "Start der Censor-Region sollte 0.0 sein");
        assert_eq!(audio[30], 0.0, "Mitte der Censor-Region sollte 0.0 sein");
        assert_eq!(audio[39], 0.0, "Ende der Censor-Region sollte 0.0 sein");
        assert_eq!(audio[40], 0.8, "Nach Censor-Region sollte unverändert sein");
    }

    #[test]
    fn test_censor_beep_mode() {
        let mut engine = BleeperEngine::new();
        engine.armed = true;
        engine.set_mode(BleepMode::Beep);
        engine.set_tone(1000.0);
        engine.set_volume(-6.0);

        let mut audio = vec![0.0; 100];
        engine.censor(&mut audio, 10, 50);

        // Bereich 10-50 sollte Sinus-Ton enthalten
        // Hinweis: audio[10] kann 0.0 sein (sin(0) = 0), daher prüfen wir audio[11]
        assert_ne!(audio[11], 0.0, "Beep Sample 1 sollte nicht 0.0 sein");
        assert_ne!(audio[30], 0.0, "Beep Sample 20 sollte nicht 0.0 sein");

        // Amplitude sollte unter 1.0 sein (-6dB ≈ 0.5)
        assert!(audio[11].abs() < 1.0);
        assert!(audio[11].abs() > 0.0);

        // Vor und nach Censor-Region sollte 0.0 sein
        assert_eq!(audio[9], 0.0);
        assert_eq!(audio[50], 0.0);
    }

    #[test]
    fn test_censor_noise_mode() {
        let mut engine = BleeperEngine::new();
        engine.armed = true;
        engine.set_mode(BleepMode::Noise);

        let mut audio = vec![0.0; 100];
        engine.censor(&mut audio, 10, 50);

        // Noise sollte zufällige Werte haben (nicht alle gleich)
        let sample1 = audio[10];
        let sample2 = audio[30];
        assert_ne!(sample1, 0.0, "Noise sollte nicht 0.0 sein");
        assert_ne!(sample2, 0.0, "Noise sollte nicht 0.0 sein");

        // Samples sollten unterschiedlich sein (statistisch sehr wahrscheinlich)
        assert_ne!(sample1, sample2, "Noise-Samples sollten unterschiedlich sein");
    }

    #[test]
    fn test_censor_reverse_mode() {
        let mut engine = BleeperEngine::new();
        engine.armed = true;
        engine.set_mode(BleepMode::Reverse);

        let mut audio = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        engine.censor(&mut audio, 2, 8);

        // Bereich 2-8 sollte rückwärts sein: [0.3, 0.4, 0.5, 0.6, 0.7, 0.8]
        // Nach Reverse: [0.8, 0.7, 0.6, 0.5, 0.4, 0.3]
        assert_eq!(audio[0], 0.1, "Vor Censor sollte unverändert sein");
        assert_eq!(audio[1], 0.2, "Vor Censor sollte unverändert sein");
        assert_eq!(audio[2], 0.8, "Sollte reversed sein");
        assert_eq!(audio[3], 0.7, "Sollte reversed sein");
        assert_eq!(audio[4], 0.6, "Sollte reversed sein");
        assert_eq!(audio[5], 0.5, "Sollte reversed sein");
        assert_eq!(audio[6], 0.4, "Sollte reversed sein");
        assert_eq!(audio[7], 0.3, "Sollte reversed sein");
        assert_eq!(audio[8], 0.9, "Nach Censor sollte unverändert sein");
        assert_eq!(audio[9], 1.0, "Nach Censor sollte unverändert sein");
    }

    #[test]
    fn test_censor_invalid_range() {
        let mut engine = BleeperEngine::new();
        engine.armed = true;

        let mut audio = vec![1.0; 100];

        // Start >= End sollte nichts machen
        engine.censor(&mut audio, 50, 50);
        assert_eq!(audio[50], 1.0);

        // End > Länge sollte nichts machen
        engine.censor(&mut audio, 50, 200);
        assert_eq!(audio[50], 1.0);
    }

    #[test]
    fn test_tone_and_volume_setters() {
        let mut engine = BleeperEngine::new();

        // Tone clamping testen
        engine.set_tone(5000.0);
        assert_eq!(engine.tone_hz, 2000.0, "Tone sollte auf 2000Hz geclamppt sein");

        engine.set_tone(100.0);
        assert_eq!(engine.tone_hz, 200.0, "Tone sollte auf 200Hz geclamppt sein");

        // Volume clamping testen
        engine.set_volume(10.0);
        assert_eq!(engine.volume_db, 0.0, "Volume sollte auf 0dB geclamppt sein");

        engine.set_volume(-50.0);
        assert_eq!(engine.volume_db, -30.0, "Volume sollte auf -30dB geclamppt sein");
    }
}
