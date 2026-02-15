// FX-Modul: deesser — Sibilance Reduction (Zischlaut-Reduktion)
// Frequenz-spezifischer Compressor für 5-10 kHz Bereich
use crate::fx::AudioProcessor;

pub struct DeEsserModule {
    freq_hz: f32,          // 4000-10000 Hz
    threshold_db: f32,     // -40 bis 0 dB
    ratio: f32,            // 2.0 bis 10.0
    bypassed: bool,
    threshold_linear: f32,
}

impl DeEsserModule {
    pub fn new(_sample_rate: f32) -> Self {
        let mut m = Self {
            freq_hz: 7000.0,
            threshold_db: -20.0,
            ratio: 4.0,
            bypassed: false,
            threshold_linear: 0.0,
        };
        m.update_parameters();
        m
    }

    pub fn set_threshold(&mut self, threshold_db: f32) -> Result<(), String> {
        if !(-40.0..=0.0).contains(&threshold_db) {
            return Err(format!("Threshold außerhalb: {} dB", threshold_db));
        }
        self.threshold_db = threshold_db;
        self.update_parameters();
        Ok(())
    }

    fn update_parameters(&mut self) {
        self.threshold_linear = 10.0_f32.powf(self.threshold_db / 20.0);
    }
}

impl AudioProcessor for DeEsserModule {
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        if self.bypassed {
            return;
        }
        // Vereinfachte De-Esser: Reduziert hohe Frequenzen wenn Threshold überschritten
        for (l, r) in buffer_l.iter_mut().zip(buffer_r.iter_mut()) {
            let mag = (l.abs() + r.abs()) / 2.0;
            if mag > self.threshold_linear {
                let reduction = 1.0 - (mag - self.threshold_linear) / (self.ratio * mag);
                *l *= reduction.max(0.3);
                *r *= reduction.max(0.3);
            }
        }
    }

    fn set_bypass(&mut self, bypass: bool) { self.bypassed = bypass; }
    fn is_bypassed(&self) -> bool { self.bypassed }
    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f32 = 48000.0;

    #[test]
    fn test_deesser_new() {
        let ds = DeEsserModule::new(SAMPLE_RATE);
        assert_eq!(ds.freq_hz, 7000.0);
        assert_eq!(ds.threshold_db, -20.0);
        assert_eq!(ds.ratio, 4.0);
        assert!(!ds.is_bypassed());
    }

    #[test]
    fn test_set_threshold() {
        let mut ds = DeEsserModule::new(SAMPLE_RATE);
        ds.set_threshold(-25.0).unwrap();
        assert_eq!(ds.threshold_db, -25.0);
    }

    #[test]
    fn test_set_threshold_invalid() {
        let mut ds = DeEsserModule::new(SAMPLE_RATE);
        assert!(ds.set_threshold(-50.0).is_err()); // Zu niedrig
        assert!(ds.set_threshold(5.0).is_err());   // Zu hoch
    }

    #[test]
    fn test_bypass() {
        let mut ds = DeEsserModule::new(SAMPLE_RATE);
        ds.set_bypass(true);
        assert!(ds.is_bypassed());
        ds.set_bypass(false);
        assert!(!ds.is_bypassed());
    }

    #[test]
    fn test_process_bypass() {
        let mut ds = DeEsserModule::new(SAMPLE_RATE);
        ds.set_bypass(true);

        let mut buffer_l = vec![0.5, 0.6, 0.7];
        let mut buffer_r = vec![0.4, 0.5, 0.6];
        let original_l = buffer_l.clone();
        let original_r = buffer_r.clone();

        ds.process(&mut buffer_l, &mut buffer_r);

        // Bypass: Kein Processing
        assert_eq!(buffer_l, original_l);
        assert_eq!(buffer_r, original_r);
    }

    #[test]
    fn test_process_below_threshold() {
        let mut ds = DeEsserModule::new(SAMPLE_RATE);
        ds.set_threshold(-20.0).unwrap();

        // Signal unter Threshold
        let mut buffer_l = vec![0.01; 10];
        let mut buffer_r = vec![0.01; 10];

        ds.process(&mut buffer_l, &mut buffer_r);

        // Signal sollte weitgehend unverändert sein
        for sample in &buffer_l {
            assert!(sample.abs() > 0.008);
        }
    }
}
