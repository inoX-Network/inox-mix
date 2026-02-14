// FX-Modul: compressor — Dynamics Compression
// Reduziert dynamischen Bereich durch Gain-Reduktion bei hohen Pegeln
use crate::fx::AudioProcessor;

pub struct CompressorModule {
    threshold_db: f32,     // -60 bis 0 dB
    ratio: f32,            // 1.0 bis 20.0
    attack_ms: f32,        // 0.1 bis 100 ms
    release_ms: f32,       // 10 bis 1000 ms
    bypassed: bool,
    sample_rate: f32,
    threshold_linear: f32,
    attack_coeff: f32,
    release_coeff: f32,
    envelope: f32,
}

impl CompressorModule {
    pub fn new(sample_rate: f32) -> Self {
        let mut m = Self {
            threshold_db: -20.0,
            ratio: 4.0,
            attack_ms: 5.0,
            release_ms: 100.0,
            bypassed: false,
            sample_rate,
            threshold_linear: 0.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            envelope: 0.0,
        };
        m.update_parameters();
        m
    }

    pub fn set_threshold(&mut self, threshold_db: f32) -> Result<(), String> {
        if !(-60.0..=0.0).contains(&threshold_db) {
            return Err(format!("Threshold außerhalb: {} dB", threshold_db));
        }
        self.threshold_db = threshold_db;
        self.update_parameters();
        Ok(())
    }

    fn update_parameters(&mut self) {
        self.threshold_linear = 10.0_f32.powf(self.threshold_db / 20.0);
        self.attack_coeff = (-1.0 / (self.attack_ms * self.sample_rate / 1000.0)).exp();
        self.release_coeff = (-1.0 / (self.release_ms * self.sample_rate / 1000.0)).exp();
    }
}

impl AudioProcessor for CompressorModule {
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        if self.bypassed {
            return;
        }
        for (l, r) in buffer_l.iter_mut().zip(buffer_r.iter_mut()) {
            let input_level = (l.abs() + r.abs()) / 2.0;
            
            // Envelope Follower
            let coeff = if input_level > self.envelope { self.attack_coeff } else { self.release_coeff };
            self.envelope = coeff * self.envelope + (1.0 - coeff) * input_level;
            
            // Gain Reduction berechnen
            let gain_reduction = if self.envelope > self.threshold_linear {
                let over_db = 20.0 * (self.envelope / self.threshold_linear).log10();
                let compressed_db = over_db / self.ratio;
                10.0_f32.powf((compressed_db - over_db) / 20.0)
            } else {
                1.0
            };
            
            *l *= gain_reduction;
            *r *= gain_reduction;
        }
    }

    fn set_bypass(&mut self, bypass: bool) { self.bypassed = bypass; }
    fn is_bypassed(&self) -> bool { self.bypassed }
    fn reset(&mut self) { self.envelope = 0.0; }
}
