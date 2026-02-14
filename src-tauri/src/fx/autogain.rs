// FX-Modul: autogain — Automatische Gain-Normalisierung basierend auf RMS-Pegel
//
// Misst den durchschnittlichen RMS-Pegel über ein Zeitfenster und passt den Gain
// automatisch an, um ein Ziel-Level zu erreichen (z.B. -18 dBFS für Broadcast).
// SPEC: 03-signal-chain

use crate::fx::AudioProcessor;

/// AutoGain-Modul für automatische Lautstärke-Normalisierung
pub struct AutoGainModule {
    /// Ziel-Level in dB (-40 bis 0 dB, Standard: -18 dB)
    target_level_db: f32,
    /// RMS-Messungs-Fenster in ms (100 bis 5000 ms, Standard: 1000 ms)
    window_ms: f32,
    /// Attack-Zeit in ms (10 bis 1000 ms, Standard: 100 ms)
    attack_ms: f32,
    /// Release-Zeit in ms (100 bis 5000 ms, Standard: 500 ms)
    release_ms: f32,
    /// Bypass-Status
    bypassed: bool,
    /// Sample-Rate (für Zeitberechnungen)
    sample_rate: f32,
    /// Aktueller Gain-Faktor (linear)
    current_gain: f32,
    /// Attack-Koeffizient (berechnet aus attack_ms)
    attack_coeff: f32,
    /// Release-Koeffizient (berechnet aus release_ms)
    release_coeff: f32,
    /// RMS-Akkumulator für L-Kanal
    rms_sum_l: f32,
    /// RMS-Akkumulator für R-Kanal
    rms_sum_r: f32,
    /// Anzahl Samples im aktuellen RMS-Fenster
    rms_sample_count: usize,
    /// RMS-Fenstergröße in Samples
    rms_window_samples: usize,
}

impl AutoGainModule {
    /// Neues AutoGain-Modul mit Standard-Parametern
    pub fn new(sample_rate: f32) -> Self {
        let mut module = Self {
            target_level_db: -18.0,
            window_ms: 1000.0,
            attack_ms: 100.0,
            release_ms: 500.0,
            bypassed: false,
            sample_rate,
            current_gain: 1.0, // Unity Gain
            attack_coeff: 0.0,
            release_coeff: 0.0,
            rms_sum_l: 0.0,
            rms_sum_r: 0.0,
            rms_sample_count: 0,
            rms_window_samples: 0,
        };
        module.update_coefficients();
        module
    }

    /// Target-Level setzen (in dB)
    pub fn set_target_level(&mut self, level_db: f32) -> Result<(), String> {
        if !(-40.0..=0.0).contains(&level_db) {
            return Err(format!(
                "Target Level außerhalb des Bereichs: {} dB (erlaubt: -40 bis 0 dB)",
                level_db
            ));
        }
        self.target_level_db = level_db;
        Ok(())
    }

    /// RMS-Fenster setzen (in ms)
    pub fn set_window(&mut self, window_ms: f32) -> Result<(), String> {
        if !(100.0..=5000.0).contains(&window_ms) {
            return Err(format!(
                "Window außerhalb des Bereichs: {} ms (erlaubt: 100 bis 5000 ms)",
                window_ms
            ));
        }
        self.window_ms = window_ms;
        self.update_coefficients();
        Ok(())
    }

    /// Attack-Zeit setzen (in ms)
    pub fn set_attack(&mut self, attack_ms: f32) -> Result<(), String> {
        if !(10.0..=1000.0).contains(&attack_ms) {
            return Err(format!(
                "Attack außerhalb des Bereichs: {} ms (erlaubt: 10 bis 1000 ms)",
                attack_ms
            ));
        }
        self.attack_ms = attack_ms;
        self.update_coefficients();
        Ok(())
    }

    /// Release-Zeit setzen (in ms)
    pub fn set_release(&mut self, release_ms: f32) -> Result<(), String> {
        if !(100.0..=5000.0).contains(&release_ms) {
            return Err(format!(
                "Release außerhalb des Bereichs: {} ms (erlaubt: 100 bis 5000 ms)",
                release_ms
            ));
        }
        self.release_ms = release_ms;
        self.update_coefficients();
        Ok(())
    }

    /// Koeffizienten neu berechnen (nach Parameter-Änderung)
    fn update_coefficients(&mut self) {
        // Attack/Release Koeffizienten (exponentielles Smoothing)
        // coeff = exp(-1 / (time_ms * sample_rate / 1000))
        self.attack_coeff = (-1.0 / (self.attack_ms * self.sample_rate / 1000.0)).exp();
        self.release_coeff = (-1.0 / (self.release_ms * self.sample_rate / 1000.0)).exp();

        // RMS-Fenstergröße in Samples
        self.rms_window_samples = (self.window_ms * self.sample_rate / 1000.0) as usize;
    }

    /// RMS-Pegel berechnen (in dB)
    fn calculate_rms_db(&self) -> f32 {
        if self.rms_sample_count == 0 {
            return -80.0; // Sehr leise wenn keine Samples
        }

        // RMS = sqrt(sum(x^2) / N)
        let rms_l = (self.rms_sum_l / self.rms_sample_count as f32).sqrt();
        let rms_r = (self.rms_sum_r / self.rms_sample_count as f32).sqrt();
        let rms = (rms_l + rms_r) / 2.0; // Durchschnitt beider Kanäle

        // dB-Konvertierung (20 * log10(rms))
        if rms > 0.0001 {
            20.0 * rms.log10()
        } else {
            -80.0 // Minimum
        }
    }
}

impl AudioProcessor for AutoGainModule {
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        if self.bypassed {
            return;
        }

        for (sample_l, sample_r) in buffer_l.iter_mut().zip(buffer_r.iter_mut()) {
            // RMS-Messung aktualisieren
            self.rms_sum_l += *sample_l * *sample_l;
            self.rms_sum_r += *sample_r * *sample_r;
            self.rms_sample_count += 1;

            // Wenn RMS-Fenster voll: Gain anpassen
            if self.rms_sample_count >= self.rms_window_samples {
                let current_rms_db = self.calculate_rms_db();

                // Ziel-Gain berechnen
                let gain_adjustment_db = self.target_level_db - current_rms_db;
                let target_gain = 10.0_f32.powf(gain_adjustment_db / 20.0);

                // Smoothing mit Attack/Release
                let coeff = if target_gain > self.current_gain {
                    self.attack_coeff // Erhöhen (Attack)
                } else {
                    self.release_coeff // Reduzieren (Release)
                };

                self.current_gain = coeff * self.current_gain + (1.0 - coeff) * target_gain;

                // Gain limitieren (-20 dB bis +20 dB)
                self.current_gain = self.current_gain.clamp(0.1, 10.0);

                // RMS-Fenster zurücksetzen
                self.rms_sum_l = 0.0;
                self.rms_sum_r = 0.0;
                self.rms_sample_count = 0;
            }

            // Gain anwenden
            *sample_l *= self.current_gain;
            *sample_r *= self.current_gain;
        }
    }

    fn set_bypass(&mut self, bypass: bool) {
        self.bypassed = bypass;
    }

    fn is_bypassed(&self) -> bool {
        self.bypassed
    }

    fn reset(&mut self) {
        self.current_gain = 1.0;
        self.rms_sum_l = 0.0;
        self.rms_sum_r = 0.0;
        self.rms_sample_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f32 = 48000.0;

    #[test]
    fn test_autogain_new() {
        let ag = AutoGainModule::new(SAMPLE_RATE);
        assert_eq!(ag.target_level_db, -18.0);
        assert_eq!(ag.window_ms, 1000.0);
        assert_eq!(ag.current_gain, 1.0);
        assert!(!ag.is_bypassed());
    }

    #[test]
    fn test_set_target_level() {
        let mut ag = AutoGainModule::new(SAMPLE_RATE);

        // Gültiger Bereich
        ag.set_target_level(-20.0).unwrap();
        assert_eq!(ag.target_level_db, -20.0);

        ag.set_target_level(-12.0).unwrap();
        assert_eq!(ag.target_level_db, -12.0);
    }

    #[test]
    fn test_set_target_level_invalid() {
        let mut ag = AutoGainModule::new(SAMPLE_RATE);

        let result = ag.set_target_level(-50.0);
        assert!(result.is_err());

        let result = ag.set_target_level(5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_window() {
        let mut ag = AutoGainModule::new(SAMPLE_RATE);

        ag.set_window(500.0).unwrap();
        assert_eq!(ag.window_ms, 500.0);

        ag.set_window(2000.0).unwrap();
        assert_eq!(ag.window_ms, 2000.0);
    }

    #[test]
    fn test_bypass() {
        let mut ag = AutoGainModule::new(SAMPLE_RATE);

        ag.set_bypass(true);
        assert!(ag.is_bypassed());

        ag.set_bypass(false);
        assert!(!ag.is_bypassed());
    }

    #[test]
    fn test_process_bypass() {
        let mut ag = AutoGainModule::new(SAMPLE_RATE);
        ag.set_bypass(true);

        let mut buffer_l = vec![0.5; 256];
        let mut buffer_r = vec![0.5; 256];

        ag.process(&mut buffer_l, &mut buffer_r);

        // Bei Bypass sollten Werte unverändert sein
        assert_eq!(buffer_l[0], 0.5);
        assert_eq!(buffer_r[0], 0.5);
    }

    #[test]
    fn test_reset() {
        let mut ag = AutoGainModule::new(SAMPLE_RATE);
        ag.current_gain = 2.0;
        ag.rms_sum_l = 10.0;
        ag.rms_sample_count = 100;

        ag.reset();

        assert_eq!(ag.current_gain, 1.0);
        assert_eq!(ag.rms_sum_l, 0.0);
        assert_eq!(ag.rms_sample_count, 0);
    }
}
