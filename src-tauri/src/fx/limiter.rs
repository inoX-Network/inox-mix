// FX-Modul: limiter — Brickwall Peak Limiter (Clipping-Schutz)
//
// Verhindert Clipping durch harte Begrenzung des Signals auf ein Ceiling-Level.
// Nutzt Look-Ahead Buffer für transparente Limitierung ohne Artefakte.
// SPEC: 03-signal-chain

use crate::fx::AudioProcessor;
use std::collections::VecDeque;

/// Limiter-Modul für Clipping-Schutz
pub struct LimiterModule {
    /// Ceiling in dB (-20 bis 0 dB, Standard: -0.3 dB)
    ceiling_db: f32,
    /// Release-Zeit in ms (10 bis 1000 ms, Standard: 50 ms)
    release_ms: f32,
    /// Bypass-Status
    bypassed: bool,
    /// Sample-Rate
    sample_rate: f32,
    /// Ceiling linear (berechnet aus ceiling_db)
    ceiling_linear: f32,
    /// Release-Koeffizient
    release_coeff: f32,
    /// Aktueller Gain-Reduktionsfaktor
    current_gain_reduction: f32,
    /// Look-Ahead Buffer L-Kanal
    lookahead_buffer_l: VecDeque<f32>,
    /// Look-Ahead Buffer R-Kanal
    lookahead_buffer_r: VecDeque<f32>,
    /// Look-Ahead Größe in Samples (5ms = 240 Samples @ 48kHz)
    lookahead_samples: usize,
}

impl LimiterModule {
    /// Neues Limiter-Modul mit Standard-Parametern
    pub fn new(sample_rate: f32) -> Self {
        let mut module = Self {
            ceiling_db: -0.3,
            release_ms: 50.0,
            bypassed: false,
            sample_rate,
            ceiling_linear: 0.0,
            release_coeff: 0.0,
            current_gain_reduction: 1.0,
            lookahead_buffer_l: VecDeque::new(),
            lookahead_buffer_r: VecDeque::new(),
            lookahead_samples: 0,
        };
        module.update_parameters();
        module
    }

    /// Ceiling setzen (in dB)
    pub fn set_ceiling(&mut self, ceiling_db: f32) -> Result<(), String> {
        if !(-20.0..=0.0).contains(&ceiling_db) {
            return Err(format!(
                "Ceiling außerhalb des Bereichs: {} dB (erlaubt: -20 bis 0 dB)",
                ceiling_db
            ));
        }
        self.ceiling_db = ceiling_db;
        self.update_parameters();
        Ok(())
    }

    /// Release setzen (in ms)
    pub fn set_release(&mut self, release_ms: f32) -> Result<(), String> {
        if !(10.0..=1000.0).contains(&release_ms) {
            return Err(format!(
                "Release außerhalb des Bereichs: {} ms (erlaubt: 10 bis 1000 ms)",
                release_ms
            ));
        }
        self.release_ms = release_ms;
        self.update_parameters();
        Ok(())
    }

    /// Parameter neu berechnen
    fn update_parameters(&mut self) {
        // Ceiling linear
        self.ceiling_linear = 10.0_f32.powf(self.ceiling_db / 20.0);

        // Release Koeffizient
        self.release_coeff = (-1.0 / (self.release_ms * self.sample_rate / 1000.0)).exp();

        // Look-Ahead Größe (5ms)
        self.lookahead_samples = (5.0 * self.sample_rate / 1000.0) as usize;
    }
}

impl AudioProcessor for LimiterModule {
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        if self.bypassed {
            return;
        }

        for (sample_l, sample_r) in buffer_l.iter_mut().zip(buffer_r.iter_mut()) {
            // Samples in Look-Ahead Buffer speichern
            self.lookahead_buffer_l.push_back(*sample_l);
            self.lookahead_buffer_r.push_back(*sample_r);

            // Wenn Look-Ahead Buffer voll, Sample verarbeiten
            if self.lookahead_buffer_l.len() > self.lookahead_samples {
                // Ältestes Sample aus Buffer holen
                let delayed_l = self.lookahead_buffer_l.pop_front().unwrap();
                let delayed_r = self.lookahead_buffer_r.pop_front().unwrap();

                // Peak im Look-Ahead Fenster finden
                let peak_l = self
                    .lookahead_buffer_l
                    .iter()
                    .map(|s| s.abs())
                    .fold(0.0_f32, f32::max);
                let peak_r = self
                    .lookahead_buffer_r
                    .iter()
                    .map(|s| s.abs())
                    .fold(0.0_f32, f32::max);
                let peak = peak_l.max(peak_r);

                // Gain-Reduktion berechnen wenn Peak > Ceiling
                let target_gain_reduction = if peak > self.ceiling_linear {
                    self.ceiling_linear / peak
                } else {
                    1.0 // Keine Reduktion
                };

                // Smooth Gain-Reduktion mit Release
                if target_gain_reduction < self.current_gain_reduction {
                    // Attack: Sofort reduzieren (instant)
                    self.current_gain_reduction = target_gain_reduction;
                } else {
                    // Release: Langsam zurück
                    self.current_gain_reduction = self.release_coeff * self.current_gain_reduction
                        + (1.0 - self.release_coeff) * target_gain_reduction;
                }

                // Gain-Reduktion anwenden
                *sample_l = delayed_l * self.current_gain_reduction;
                *sample_r = delayed_r * self.current_gain_reduction;
            } else {
                // Look-Ahead Buffer füllt sich noch, Output = 0
                *sample_l = 0.0;
                *sample_r = 0.0;
            }
        }
    }

    fn set_bypass(&mut self, bypass: bool) {
        self.bypassed = bypass;
    }

    fn is_bypassed(&self) -> bool {
        self.bypassed
    }

    fn reset(&mut self) {
        self.current_gain_reduction = 1.0;
        self.lookahead_buffer_l.clear();
        self.lookahead_buffer_r.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f32 = 48000.0;

    #[test]
    fn test_limiter_new() {
        let lim = LimiterModule::new(SAMPLE_RATE);
        assert_eq!(lim.ceiling_db, -0.3);
        assert_eq!(lim.release_ms, 50.0);
        assert!(!lim.is_bypassed());
    }

    #[test]
    fn test_set_ceiling() {
        let mut lim = LimiterModule::new(SAMPLE_RATE);
        lim.set_ceiling(-1.0).unwrap();
        assert_eq!(lim.ceiling_db, -1.0);
    }

    #[test]
    fn test_set_ceiling_invalid() {
        let mut lim = LimiterModule::new(SAMPLE_RATE);
        assert!(lim.set_ceiling(-25.0).is_err());
        assert!(lim.set_ceiling(5.0).is_err());
    }

    #[test]
    fn test_bypass() {
        let mut lim = LimiterModule::new(SAMPLE_RATE);
        lim.set_bypass(true);
        assert!(lim.is_bypassed());
    }

    #[test]
    fn test_reset() {
        let mut lim = LimiterModule::new(SAMPLE_RATE);
        lim.current_gain_reduction = 0.5;
        lim.lookahead_buffer_l.push_back(1.0);

        lim.reset();

        assert_eq!(lim.current_gain_reduction, 1.0);
        assert!(lim.lookahead_buffer_l.is_empty());
    }
}
