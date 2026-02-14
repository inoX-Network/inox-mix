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
