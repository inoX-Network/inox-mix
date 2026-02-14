// FX-Modul: denoise — Spectral Gate Noise Reduction
// Einfache Rauschunterdrückung durch Noise Floor Threshold
use crate::fx::AudioProcessor;

pub struct DenoiseModule {
    threshold_db: f32,     // -60 bis -10 dB
    reduction_db: f32,     // 0 bis 40 dB
    bypassed: bool,
    threshold_linear: f32,
    reduction_factor: f32,
}

impl DenoiseModule {
    pub fn new(_sample_rate: f32) -> Self {
        let mut m = Self {
            threshold_db: -40.0,
            reduction_db: 20.0,
            bypassed: false,
            threshold_linear: 0.0,
            reduction_factor: 0.0,
        };
        m.update_parameters();
        m
    }

    pub fn set_threshold(&mut self, threshold_db: f32) -> Result<(), String> {
        if !(-60.0..=-10.0).contains(&threshold_db) {
            return Err(format!("Threshold außerhalb: {} dB", threshold_db));
        }
        self.threshold_db = threshold_db;
        self.update_parameters();
        Ok(())
    }

    fn update_parameters(&mut self) {
        self.threshold_linear = 10.0_f32.powf(self.threshold_db / 20.0);
        self.reduction_factor = 10.0_f32.powf(-self.reduction_db / 20.0);
    }
}

impl AudioProcessor for DenoiseModule {
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        if self.bypassed {
            return;
        }
        for (l, r) in buffer_l.iter_mut().zip(buffer_r.iter_mut()) {
            let mag = (l.abs() + r.abs()) / 2.0;
            if mag < self.threshold_linear {
                *l *= self.reduction_factor;
                *r *= self.reduction_factor;
            }
        }
    }

    fn set_bypass(&mut self, bypass: bool) { self.bypassed = bypass; }
    fn is_bypassed(&self) -> bool { self.bypassed }
    fn reset(&mut self) {}
}
