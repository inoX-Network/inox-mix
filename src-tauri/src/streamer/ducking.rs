// Modul: streamer/ducking — Sidechain-Ducking (Musik leiser bei Sprache)
use serde::{Deserialize, Serialize};

/// Ducking-Parameter für Sidechain-Kompression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuckingParams {
    /// Ducking-Stärke in dB (0 = aus, -20 = stark)
    pub amount_db: f32,
    /// Attack-Zeit in ms
    pub attack_ms: f32,
    /// Release-Zeit in ms
    pub release_ms: f32,
    /// Schwellwert in dB (ab wann geduckt wird)
    pub threshold_db: f32,
}

/// Sidechain-Ducking Engine
#[derive(Debug)]
pub struct DuckingEngine {
    /// Aktuelle Parameter
    pub params: DuckingParams,
    /// Ducking aktiv
    pub enabled: bool,
    /// Sample-Rate (für Attack/Release Berechnung)
    sample_rate: f32,
    /// Current envelope level (0.0 = full duck, 1.0 = no duck)
    envelope: f32,
    /// Attack-Koeffizient (vorberechnet)
    attack_coeff: f32,
    /// Release-Koeffizient (vorberechnet)
    release_coeff: f32,
}

impl DuckingEngine {
    /// Neue Ducking-Engine erstellen
    pub fn new() -> Self {
        let sample_rate = 48000.0; // Standard Sample-Rate
        let attack_ms = 50.0;
        let release_ms = 500.0;

        Self {
            params: DuckingParams {
                amount_db: -15.0, // Standard: -15dB Reduktion
                attack_ms,
                release_ms,
                threshold_db: -30.0, // -30dB Threshold
            },
            enabled: false,
            sample_rate,
            envelope: 1.0, // Start ohne Ducking
            attack_coeff: Self::calculate_coeff(sample_rate, attack_ms),
            release_coeff: Self::calculate_coeff(sample_rate, release_ms),
        }
    }

    /// Attack/Release Koeffizient berechnen
    /// Formula: exp(-1.0 / (sample_rate * time_seconds))
    fn calculate_coeff(sample_rate: f32, time_ms: f32) -> f32 {
        let time_samples = sample_rate * (time_ms / 1000.0);
        (-1.0 / time_samples).exp()
    }

    /// Ducking-Parameter setzen
    pub fn set_amount(&mut self, amount_db: f32) {
        self.params.amount_db = amount_db.clamp(-30.0, 0.0);
    }

    pub fn set_attack(&mut self, attack_ms: f32) {
        self.params.attack_ms = attack_ms.clamp(10.0, 500.0);
        self.attack_coeff = Self::calculate_coeff(self.sample_rate, self.params.attack_ms);
    }

    pub fn set_release(&mut self, release_ms: f32) {
        self.params.release_ms = release_ms.clamp(50.0, 2000.0);
        self.release_coeff = Self::calculate_coeff(self.sample_rate, self.params.release_ms);
    }

    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.params.threshold_db = threshold_db.clamp(-50.0, 0.0);
    }

    /// Ducking auf Audio-Buffer anwenden (Sidechain-Kompression)
    ///
    /// # Argumente
    /// * `audio` - Audio-Buffer der geduckt werden soll (z.B. Musik)
    /// * `sidechain` - Sidechain-Signal (z.B. Mikrofon/Voice)
    ///
    /// # Phase 2: Echte Sidechain-Analyse
    /// - Envelope-Follower auf Sidechain-Signal (RMS)
    /// - Attack/Release Envelope berechnen
    /// - Gain-Reduktion anwenden wenn Threshold überschritten
    pub fn process(&mut self, audio: &mut [f32], sidechain: &[f32]) {
        if !self.enabled || audio.is_empty() || sidechain.is_empty() {
            return;
        }

        // Sidechain-Signal analysieren (RMS über alle Samples)
        let sidechain_rms = Self::calculate_rms(sidechain);
        let sidechain_db = Self::linear_to_db(sidechain_rms);

        // Prüfen ob Sidechain über Threshold liegt
        let should_duck = sidechain_db > self.params.threshold_db;

        // Target envelope level berechnen
        let target_envelope = if should_duck {
            // Duck: Gain-Reduktion anwenden
            Self::db_to_linear(self.params.amount_db)
        } else {
            // Kein Duck: Volle Lautstärke
            1.0
        };

        // Envelope mit Attack/Release smoothen
        for sample in audio.iter_mut() {
            // Attack (wenn Envelope sinkt) oder Release (wenn Envelope steigt)
            let coeff = if target_envelope < self.envelope {
                self.attack_coeff // Attack (schneller)
            } else {
                self.release_coeff // Release (langsamer)
            };

            // Envelope-Follower: Exponential Smoothing
            self.envelope = coeff * self.envelope + (1.0 - coeff) * target_envelope;

            // Gain-Reduktion anwenden
            *sample *= self.envelope;
        }
    }

    /// RMS (Root Mean Square) eines Audio-Buffers berechnen
    fn calculate_rms(buffer: &[f32]) -> f32 {
        if buffer.is_empty() {
            return 0.0;
        }

        let sum_squares: f32 = buffer.iter().map(|&s| s * s).sum();
        (sum_squares / buffer.len() as f32).sqrt()
    }

    /// Linear-Amplitude zu dB konvertieren
    fn linear_to_db(linear: f32) -> f32 {
        if linear <= 0.0 {
            -100.0 // -∞ dB
        } else {
            20.0 * linear.log10()
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
    fn test_ducking_engine_creation() {
        let engine = DuckingEngine::new();
        assert_eq!(engine.enabled, false);
        assert_eq!(engine.envelope, 1.0); // Start ohne Ducking
        assert!(engine.attack_coeff > 0.0 && engine.attack_coeff < 1.0);
        assert!(engine.release_coeff > 0.0 && engine.release_coeff < 1.0);
    }

    #[test]
    fn test_calculate_rms() {
        // Stilles Signal
        let silent = vec![0.0; 100];
        assert_eq!(DuckingEngine::calculate_rms(&silent), 0.0);

        // Maximales Signal
        let loud = vec![1.0; 100];
        assert!((DuckingEngine::calculate_rms(&loud) - 1.0).abs() < 0.001);

        // Halbe Amplitude
        let medium = vec![0.5; 100];
        assert!((DuckingEngine::calculate_rms(&medium) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_db_conversions() {
        // 0 dB = 1.0 linear
        assert!((DuckingEngine::db_to_linear(0.0) - 1.0).abs() < 0.001);

        // -6 dB ≈ 0.5 linear
        assert!((DuckingEngine::db_to_linear(-6.0) - 0.5).abs() < 0.01);

        // -20 dB ≈ 0.1 linear
        assert!((DuckingEngine::db_to_linear(-20.0) - 0.1).abs() < 0.001);

        // Linear zu dB und zurück
        let linear = 0.7;
        let db = DuckingEngine::linear_to_db(linear);
        let back = DuckingEngine::db_to_linear(db);
        assert!((back - linear).abs() < 0.001);
    }

    #[test]
    fn test_ducking_disabled() {
        let mut engine = DuckingEngine::new();
        engine.enabled = false;

        let mut audio = vec![1.0; 100];
        let sidechain = vec![0.8; 100]; // Lautes Sidechain

        engine.process(&mut audio, &sidechain);

        // Audio sollte unverändert sein (kein Ducking)
        assert_eq!(audio[0], 1.0);
        assert_eq!(audio[99], 1.0);
    }

    #[test]
    fn test_ducking_with_loud_sidechain() {
        let mut engine = DuckingEngine::new();
        engine.enabled = true;
        engine.params.threshold_db = -30.0;
        engine.params.amount_db = -10.0;

        let mut audio = vec![1.0; 1000]; // Volle Lautstärke
        let sidechain = vec![0.5; 1000]; // Lautes Sidechain (über Threshold)

        engine.process(&mut audio, &sidechain);

        // Audio sollte geduckt sein (< 1.0)
        // (exakter Wert hängt von Attack/Release ab)
        assert!(audio[999] < 1.0, "Audio sollte geduckt sein");
        assert!(audio[999] > 0.1, "Audio sollte nicht komplett stumm sein");
    }

    #[test]
    fn test_ducking_with_quiet_sidechain() {
        let mut engine = DuckingEngine::new();
        engine.enabled = true;
        engine.params.threshold_db = -30.0;

        let mut audio = vec![1.0; 1000];
        let sidechain = vec![0.001; 1000]; // Sehr leises Sidechain (unter Threshold)

        engine.process(&mut audio, &sidechain);

        // Audio sollte NICHT geduckt sein (envelope bleibt bei 1.0)
        assert!(
            (audio[999] - 1.0).abs() < 0.01,
            "Audio sollte nicht geduckt sein bei leisem Sidechain"
        );
    }

    #[test]
    fn test_attack_release_coefficients() {
        let mut engine = DuckingEngine::new();

        // Attack ändern
        engine.set_attack(100.0);
        let attack1 = engine.attack_coeff;

        engine.set_attack(200.0);
        let attack2 = engine.attack_coeff;

        // Längere Attack-Zeit = höherer Koeffizient (langsamerer Attack)
        assert!(attack2 > attack1);

        // Release ändern
        engine.set_release(500.0);
        let release1 = engine.release_coeff;

        engine.set_release(1000.0);
        let release2 = engine.release_coeff;

        // Längere Release-Zeit = höherer Koeffizient (langsamerer Release)
        assert!(release2 > release1);
    }
}
