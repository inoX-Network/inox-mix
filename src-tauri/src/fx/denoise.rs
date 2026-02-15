// FX-Modul: denoise — Spectral Gate Noise Reduction
// Einfache Rauschunterdrückung durch Noise Floor Threshold
use crate::fx::AudioProcessor;

pub struct DenoiseModule {
    threshold_db: f32, // -60 bis -10 dB
    reduction_db: f32, // 0 bis 40 dB
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

    fn set_bypass(&mut self, bypass: bool) {
        self.bypassed = bypass;
    }
    fn is_bypassed(&self) -> bool {
        self.bypassed
    }
    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f32 = 48000.0;

    #[test]
    fn test_denoise_new() {
        let dn = DenoiseModule::new(SAMPLE_RATE);
        assert_eq!(dn.threshold_db, -40.0);
        assert_eq!(dn.reduction_db, 20.0);
        assert!(!dn.is_bypassed());
    }

    #[test]
    fn test_set_threshold() {
        let mut dn = DenoiseModule::new(SAMPLE_RATE);
        dn.set_threshold(-30.0).unwrap();
        assert_eq!(dn.threshold_db, -30.0);
    }

    #[test]
    fn test_set_threshold_invalid() {
        let mut dn = DenoiseModule::new(SAMPLE_RATE);
        assert!(dn.set_threshold(-70.0).is_err()); // Zu niedrig
        assert!(dn.set_threshold(-5.0).is_err()); // Zu hoch
    }

    #[test]
    fn test_bypass() {
        let mut dn = DenoiseModule::new(SAMPLE_RATE);
        dn.set_bypass(true);
        assert!(dn.is_bypassed());
        dn.set_bypass(false);
        assert!(!dn.is_bypassed());
    }

    #[test]
    fn test_process_bypass() {
        let mut dn = DenoiseModule::new(SAMPLE_RATE);
        dn.set_bypass(true);

        let mut buffer_l = vec![0.001, 0.002, 0.003];
        let mut buffer_r = vec![0.001, 0.002, 0.003];
        let original_l = buffer_l.clone();
        let original_r = buffer_r.clone();

        dn.process(&mut buffer_l, &mut buffer_r);

        // Bypass: Kein Processing
        assert_eq!(buffer_l, original_l);
        assert_eq!(buffer_r, original_r);
    }

    #[test]
    fn test_process_below_threshold() {
        let mut dn = DenoiseModule::new(SAMPLE_RATE);
        dn.set_threshold(-40.0).unwrap();

        // Signal unter Threshold (0.001 = -60 dB)
        let mut buffer_l = vec![0.001; 5];
        let mut buffer_r = vec![0.001; 5];

        dn.process(&mut buffer_l, &mut buffer_r);

        // Signal sollte reduziert sein
        for sample in &buffer_l {
            assert!(sample.abs() < 0.001); // Reduziert
        }
    }

    #[test]
    fn test_process_above_threshold() {
        let mut dn = DenoiseModule::new(SAMPLE_RATE);
        dn.set_threshold(-40.0).unwrap();

        // Signal über Threshold (0.1 = -20 dB)
        let mut buffer_l = vec![0.1; 5];
        let mut buffer_r = vec![0.1; 5];
        let original_l = buffer_l.clone();

        dn.process(&mut buffer_l, &mut buffer_r);

        // Signal sollte unverändert sein
        assert_eq!(buffer_l, original_l);
    }
}
