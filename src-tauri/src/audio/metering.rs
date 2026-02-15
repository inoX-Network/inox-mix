// Modul: audio/metering — VU-Meter, Peak- und RMS-Berechnung
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Minimum dB-Wert (Stille)
const MIN_DB: f32 = -60.0;
/// Peak-Hold Dauer in Frames (bei 60fps ≈ 1.5 Sekunden)
const PEAK_HOLD_FRAMES: u32 = 90;
/// Peak-Fallrate pro Frame in dB
const PEAK_FALL_RATE: f32 = 0.5;

/// Stereo-Messwerte für einen Strip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripLevels {
    /// Strip-ID
    pub strip_id: String,
    /// Peak-Pegel linker Kanal in dB
    pub peak_l: f32,
    /// Peak-Pegel rechter Kanal in dB
    pub peak_r: f32,
    /// RMS-Pegel linker Kanal in dB
    pub rms_l: f32,
    /// RMS-Pegel rechter Kanal in dB
    pub rms_r: f32,
    /// Clipping erkannt
    pub clipping: bool,
}

/// Interner Meter-State pro Strip
#[derive(Debug)]
struct MeterState {
    /// Letzter Peak-Wert links (linear)
    peak_l: f32,
    /// Letzter Peak-Wert rechts (linear)
    peak_r: f32,
    /// Letzter RMS-Wert links (linear)
    rms_l: f32,
    /// Letzter RMS-Wert rechts (linear)
    rms_r: f32,
    /// Peak-Hold Zähler links
    peak_hold_l: u32,
    /// Peak-Hold Zähler rechts
    peak_hold_r: u32,
    /// Gehaltener Peak-Wert links (linear)
    held_peak_l: f32,
    /// Gehaltener Peak-Wert rechts (linear)
    held_peak_r: f32,
    /// Clipping erkannt
    clipping: bool,
}

impl MeterState {
    /// Neuen Meter-State erstellen
    fn new() -> Self {
        Self {
            peak_l: 0.0,
            peak_r: 0.0,
            rms_l: 0.0,
            rms_r: 0.0,
            peak_hold_l: 0,
            peak_hold_r: 0,
            held_peak_l: 0.0,
            held_peak_r: 0.0,
            clipping: false,
        }
    }
}

/// Metering-Engine berechnet Pegel für alle Kanäle
#[derive(Debug)]
pub struct MeteringEngine {
    /// Meter-State pro Strip (Key: Strip-ID)
    meters: HashMap<String, MeterState>,
}

impl MeteringEngine {
    /// Neue Metering-Engine erstellen
    pub fn new() -> Self {
        Self {
            meters: HashMap::new(),
        }
    }

    /// Strip für Metering registrieren
    pub fn register_strip(&mut self, strip_id: &str) {
        self.meters
            .entry(strip_id.to_string())
            .or_insert_with(MeterState::new);
    }

    /// Strip aus Metering entfernen
    pub fn unregister_strip(&mut self, strip_id: &str) {
        self.meters.remove(strip_id);
    }

    /// Audio-Buffer verarbeiten und Peak/RMS berechnen
    ///
    /// Erwartet interleaved Stereo-Samples (L, R, L, R, ...).
    /// Bei Mono-Signal wird der gleiche Wert für beide Kanäle verwendet.
    pub fn process_buffer(&mut self, strip_id: &str, samples: &[f32], channels: u32) {
        let meter = self
            .meters
            .entry(strip_id.to_string())
            .or_insert_with(MeterState::new);

        if samples.is_empty() {
            return;
        }

        if channels >= 2 {
            // Stereo: interleaved L, R, L, R, ...
            let (peak_l, rms_l) = calculate_peak_rms_channel(samples, 0, 2);
            let (peak_r, rms_r) = calculate_peak_rms_channel(samples, 1, 2);
            update_meter(meter, peak_l, peak_r, rms_l, rms_r);
        } else {
            // Mono: gleicher Wert für beide Kanäle
            let (peak, rms) = calculate_peak_rms(samples);
            update_meter(meter, peak, peak, rms, rms);
        }
    }

    /// Aktuelle Messwerte für alle registrierten Strips abrufen
    pub fn get_levels(&self) -> Vec<StripLevels> {
        self.meters
            .iter()
            .map(|(id, meter)| StripLevels {
                strip_id: id.clone(),
                peak_l: linear_to_db(meter.held_peak_l),
                peak_r: linear_to_db(meter.held_peak_r),
                rms_l: linear_to_db(meter.rms_l),
                rms_r: linear_to_db(meter.rms_r),
                clipping: meter.clipping,
            })
            .collect()
    }

    /// Messwerte für einen bestimmten Strip abrufen
    pub fn get_strip_levels(&self, strip_id: &str) -> Option<StripLevels> {
        self.meters.get(strip_id).map(|meter| StripLevels {
            strip_id: strip_id.to_string(),
            peak_l: linear_to_db(meter.held_peak_l),
            peak_r: linear_to_db(meter.held_peak_r),
            rms_l: linear_to_db(meter.rms_l),
            rms_r: linear_to_db(meter.rms_r),
            clipping: meter.clipping,
        })
    }

    /// Clipping-Flag für einen Strip zurücksetzen
    pub fn reset_clipping(&mut self, strip_id: &str) {
        if let Some(meter) = self.meters.get_mut(strip_id) {
            meter.clipping = false;
        }
    }

    /// Anzahl registrierter Strips
    pub fn strip_count(&self) -> usize {
        self.meters.len()
    }
}

/// Peak und RMS für einen Kanal aus interleaved Samples berechnen
fn calculate_peak_rms_channel(
    samples: &[f32],
    channel_offset: usize,
    channel_count: usize,
) -> (f32, f32) {
    let mut peak: f32 = 0.0;
    let mut sum_squares: f32 = 0.0;
    let mut count: usize = 0;

    let mut i = channel_offset;
    while i < samples.len() {
        let sample = samples[i].abs();
        if sample > peak {
            peak = sample;
        }
        sum_squares += samples[i] * samples[i];
        count += 1;
        i += channel_count;
    }

    let rms = if count > 0 {
        (sum_squares / count as f32).sqrt()
    } else {
        0.0
    };

    (peak, rms)
}

/// Peak und RMS für einen Mono-Buffer berechnen
fn calculate_peak_rms(samples: &[f32]) -> (f32, f32) {
    if samples.is_empty() {
        return (0.0, 0.0);
    }

    let mut peak: f32 = 0.0;
    let mut sum_squares: f32 = 0.0;

    for &sample in samples {
        let abs = sample.abs();
        if abs > peak {
            peak = abs;
        }
        sum_squares += sample * sample;
    }

    let rms = (sum_squares / samples.len() as f32).sqrt();
    (peak, rms)
}

/// Meter-State mit neuen Werten aktualisieren (Peak-Hold + Fall)
fn update_meter(meter: &mut MeterState, peak_l: f32, peak_r: f32, rms_l: f32, rms_r: f32) {
    // RMS direkt übernehmen
    meter.rms_l = rms_l;
    meter.rms_r = rms_r;

    // Peak aktualisieren
    meter.peak_l = peak_l;
    meter.peak_r = peak_r;

    // Clipping prüfen (Signal >= 1.0)
    if peak_l >= 1.0 || peak_r >= 1.0 {
        meter.clipping = true;
    }

    // Peak-Hold links
    if peak_l >= meter.held_peak_l {
        meter.held_peak_l = peak_l;
        meter.peak_hold_l = PEAK_HOLD_FRAMES;
    } else if meter.peak_hold_l > 0 {
        meter.peak_hold_l -= 1;
    } else {
        // Peak langsam abfallen lassen
        let fall = db_to_linear(-PEAK_FALL_RATE);
        meter.held_peak_l *= fall;
    }

    // Peak-Hold rechts
    if peak_r >= meter.held_peak_r {
        meter.held_peak_r = peak_r;
        meter.peak_hold_r = PEAK_HOLD_FRAMES;
    } else if meter.peak_hold_r > 0 {
        meter.peak_hold_r -= 1;
    } else {
        let fall = db_to_linear(-PEAK_FALL_RATE);
        meter.held_peak_r *= fall;
    }
}

/// Linearen Wert in dB umrechnen
fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0 {
        MIN_DB
    } else {
        20.0 * linear.log10()
    }
}

/// dB-Wert in linearen Faktor umrechnen
fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metering_engine_new() {
        let engine = MeteringEngine::new();
        assert_eq!(engine.strip_count(), 0);
    }

    #[test]
    fn test_register_strip() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("hw-mic");
        assert_eq!(engine.strip_count(), 1);
    }

    #[test]
    fn test_unregister_strip() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("hw-mic");
        engine.unregister_strip("hw-mic");
        assert_eq!(engine.strip_count(), 0);
    }

    #[test]
    fn test_process_mono_silence() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("test");

        let silence = vec![0.0_f32; 256];
        engine.process_buffer("test", &silence, 1);

        let levels = engine.get_strip_levels("test").unwrap();
        assert_eq!(levels.peak_l, MIN_DB);
        assert_eq!(levels.rms_l, MIN_DB);
        assert!(!levels.clipping);
    }

    #[test]
    fn test_process_mono_signal() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("test");

        // Sinus-ähnliches Signal: 0.5 Amplitude
        let signal: Vec<f32> = (0..256)
            .map(|i| 0.5 * (2.0 * std::f32::consts::PI * i as f32 / 256.0).sin())
            .collect();

        engine.process_buffer("test", &signal, 1);

        let levels = engine.get_strip_levels("test").unwrap();
        // Peak sollte bei ca. 0.5 liegen → ca. -6 dB
        assert!(
            levels.peak_l > -7.0 && levels.peak_l < -5.0,
            "Peak sollte ca. -6 dB sein, war: {}",
            levels.peak_l
        );
        // RMS sollte niedriger als Peak sein
        assert!(levels.rms_l < levels.peak_l);
        assert!(!levels.clipping);
    }

    #[test]
    fn test_process_stereo() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("test");

        // Interleaved Stereo: L=0.5, R=0.25
        let mut stereo = Vec::with_capacity(512);
        for _ in 0..256 {
            stereo.push(0.5_f32); // L
            stereo.push(0.25_f32); // R
        }

        engine.process_buffer("test", &stereo, 2);

        let levels = engine.get_strip_levels("test").unwrap();
        // Links: 0.5 → ca. -6 dB
        assert!(
            levels.peak_l > -7.0 && levels.peak_l < -5.0,
            "Peak L sollte ca. -6 dB sein, war: {}",
            levels.peak_l
        );
        // Rechts: 0.25 → ca. -12 dB
        assert!(
            levels.peak_r > -13.0 && levels.peak_r < -11.0,
            "Peak R sollte ca. -12 dB sein, war: {}",
            levels.peak_r
        );
    }

    #[test]
    fn test_clipping_detection() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("test");

        // Signal mit Clipping (>= 1.0)
        let signal = vec![1.0_f32; 256];
        engine.process_buffer("test", &signal, 1);

        let levels = engine.get_strip_levels("test").unwrap();
        assert!(levels.clipping, "Clipping sollte erkannt werden");
    }

    #[test]
    fn test_reset_clipping() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("test");

        let signal = vec![1.0_f32; 256];
        engine.process_buffer("test", &signal, 1);
        assert!(engine.get_strip_levels("test").unwrap().clipping);

        engine.reset_clipping("test");
        assert!(!engine.get_strip_levels("test").unwrap().clipping);
    }

    #[test]
    fn test_get_levels_all() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("strip-1");
        engine.register_strip("strip-2");

        let levels = engine.get_levels();
        assert_eq!(levels.len(), 2);
    }

    #[test]
    fn test_process_empty_buffer() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("test");

        engine.process_buffer("test", &[], 1);
        // Sollte nicht paniken
        let levels = engine.get_strip_levels("test").unwrap();
        assert_eq!(levels.peak_l, MIN_DB);
    }

    #[test]
    fn test_calculate_peak_rms() {
        let samples = vec![0.0, 0.5, -0.3, 0.8, -0.1];
        let (peak, rms) = calculate_peak_rms(&samples);
        assert_eq!(peak, 0.8);
        assert!(rms > 0.0 && rms < peak);
    }

    #[test]
    fn test_linear_to_db_conversion() {
        // 1.0 → 0 dB
        assert!((linear_to_db(1.0) - 0.0).abs() < 0.001);
        // 0.5 → ca. -6 dB
        assert!((linear_to_db(0.5) - (-6.02)).abs() < 0.1);
        // 0.0 → MIN_DB
        assert_eq!(linear_to_db(0.0), MIN_DB);
    }

    #[test]
    fn test_db_to_linear_conversion() {
        // 0 dB → 1.0
        assert!((db_to_linear(0.0) - 1.0).abs() < 0.001);
        // -6 dB → ca. 0.5
        assert!((db_to_linear(-6.0) - 0.501).abs() < 0.01);
    }

    #[test]
    fn test_strip_levels_serialize() {
        let levels = StripLevels {
            strip_id: "test".to_string(),
            peak_l: -6.0,
            peak_r: -12.0,
            rms_l: -10.0,
            rms_r: -16.0,
            clipping: false,
        };
        let json = serde_json::to_string(&levels);
        assert!(json.is_ok());
        assert!(json.unwrap().contains("test"));
    }
}
