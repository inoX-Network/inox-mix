// FX-Modul: eq — 3-Band Parametric Equalizer
//
// Peaking EQ mit 3 Bändern (Low, Mid, High)
// Jedes Band: Frequenz, Gain, Q-Faktor
// SPEC: 03-signal-chain

use crate::fx::AudioProcessor;

const SAMPLE_RATE: f32 = 48000.0;

/// Biquad Filter State (pro Channel)
#[derive(Debug, Clone, Copy)]
struct BiquadState {
    x1: f32, // Input delayed by 1 sample
    x2: f32, // Input delayed by 2 samples
    y1: f32, // Output delayed by 1 sample
    y2: f32, // Output delayed by 2 samples
}

impl Default for BiquadState {
    fn default() -> Self {
        Self { x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0 }
    }
}

/// Einzelnes EQ-Band (Peaking Filter)
#[derive(Debug, Clone)]
struct EqBand {
    freq_hz: f32,
    gain_db: f32,
    q: f32,
    // Biquad-Koeffizienten
    a0: f32,
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
}

impl EqBand {
    fn new(freq_hz: f32, gain_db: f32, q: f32) -> Self {
        let mut band = Self {
            freq_hz,
            gain_db,
            q,
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
        };
        band.update_coefficients();
        band
    }

    /// Biquad-Koeffizienten berechnen (Peaking EQ)
    fn update_coefficients(&mut self) {
        let omega = 2.0 * std::f32::consts::PI * self.freq_hz / SAMPLE_RATE;
        let sn = omega.sin();
        let cs = omega.cos();
        let a_gain = 10.0_f32.powf(self.gain_db / 40.0); // Gain in linear
        let alpha = sn / (2.0 * self.q);

        let b0 = 1.0 + alpha * a_gain;
        let b1_raw = -2.0 * cs;
        let b2 = 1.0 - alpha * a_gain;
        let a0_raw = 1.0 + alpha / a_gain;
        let a1_raw = -2.0 * cs;
        let a2_raw = 1.0 - alpha / a_gain;

        // Normalisieren durch a0
        self.a0 = b0 / a0_raw;
        self.a1 = b1_raw / a0_raw;
        self.a2 = b2 / a0_raw;
        self.b1 = a1_raw / a0_raw;
        self.b2 = a2_raw / a0_raw;
    }

    /// Einzelnes Sample filtern (Biquad Direct Form I)
    #[inline]
    fn process_sample(&self, input: f32, state: &mut BiquadState) -> f32 {
        let output = self.a0 * input + self.a1 * state.x1 + self.a2 * state.x2
            - self.b1 * state.y1 - self.b2 * state.y2;

        // State update
        state.x2 = state.x1;
        state.x1 = input;
        state.y2 = state.y1;
        state.y1 = output;

        output
    }
}

/// 3-Band Parametric Equalizer
pub struct EqModule {
    low: EqBand,
    mid: EqBand,
    high: EqBand,
    bypassed: bool,
    // Filter States (6 States: 3 Bänder × 2 Kanäle)
    low_state_l: BiquadState,
    low_state_r: BiquadState,
    mid_state_l: BiquadState,
    mid_state_r: BiquadState,
    high_state_l: BiquadState,
    high_state_r: BiquadState,
}

impl EqModule {
    /// Neues EQ-Modul mit Standard-Einstellungen
    pub fn new(_sample_rate: f32) -> Self {
        Self {
            low: EqBand::new(80.0, 0.0, 1.0),      // 80 Hz, 0 dB, Q=1.0
            mid: EqBand::new(1000.0, 0.0, 1.0),    // 1 kHz, 0 dB, Q=1.0
            high: EqBand::new(8000.0, 0.0, 1.0),   // 8 kHz, 0 dB, Q=1.0
            bypassed: false,
            low_state_l: BiquadState::default(),
            low_state_r: BiquadState::default(),
            mid_state_l: BiquadState::default(),
            mid_state_r: BiquadState::default(),
            high_state_l: BiquadState::default(),
            high_state_r: BiquadState::default(),
        }
    }

    /// Low-Band setzen (Frequenz, Gain, Q)
    pub fn set_low(&mut self, freq_hz: f32, gain_db: f32, q: f32) -> Result<(), String> {
        if !(20.0..=20000.0).contains(&freq_hz) {
            return Err(format!("Low Freq außerhalb: {} Hz", freq_hz));
        }
        if !(-12.0..=12.0).contains(&gain_db) {
            return Err(format!("Low Gain außerhalb: {} dB", gain_db));
        }
        if !(0.5..=5.0).contains(&q) {
            return Err(format!("Low Q außerhalb: {}", q));
        }
        self.low.freq_hz = freq_hz;
        self.low.gain_db = gain_db;
        self.low.q = q;
        self.low.update_coefficients();
        Ok(())
    }

    /// Mid-Band setzen (Frequenz, Gain, Q)
    pub fn set_mid(&mut self, freq_hz: f32, gain_db: f32, q: f32) -> Result<(), String> {
        if !(20.0..=20000.0).contains(&freq_hz) {
            return Err(format!("Mid Freq außerhalb: {} Hz", freq_hz));
        }
        if !(-12.0..=12.0).contains(&gain_db) {
            return Err(format!("Mid Gain außerhalb: {} dB", gain_db));
        }
        if !(0.5..=5.0).contains(&q) {
            return Err(format!("Mid Q außerhalb: {}", q));
        }
        self.mid.freq_hz = freq_hz;
        self.mid.gain_db = gain_db;
        self.mid.q = q;
        self.mid.update_coefficients();
        Ok(())
    }

    /// High-Band setzen (Frequenz, Gain, Q)
    pub fn set_high(&mut self, freq_hz: f32, gain_db: f32, q: f32) -> Result<(), String> {
        if !(20.0..=20000.0).contains(&freq_hz) {
            return Err(format!("High Freq außerhalb: {} Hz", freq_hz));
        }
        if !(-12.0..=12.0).contains(&gain_db) {
            return Err(format!("High Gain außerhalb: {} dB", gain_db));
        }
        if !(0.5..=5.0).contains(&q) {
            return Err(format!("High Q außerhalb: {}", q));
        }
        self.high.freq_hz = freq_hz;
        self.high.gain_db = gain_db;
        self.high.q = q;
        self.high.update_coefficients();
        Ok(())
    }
}

impl AudioProcessor for EqModule {
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        if self.bypassed {
            return;
        }

        for i in 0..buffer_l.len() {
            // Low Band
            buffer_l[i] = self.low.process_sample(buffer_l[i], &mut self.low_state_l);
            buffer_r[i] = self.low.process_sample(buffer_r[i], &mut self.low_state_r);

            // Mid Band
            buffer_l[i] = self.mid.process_sample(buffer_l[i], &mut self.mid_state_l);
            buffer_r[i] = self.mid.process_sample(buffer_r[i], &mut self.mid_state_r);

            // High Band
            buffer_l[i] = self.high.process_sample(buffer_l[i], &mut self.high_state_l);
            buffer_r[i] = self.high.process_sample(buffer_r[i], &mut self.high_state_r);
        }
    }

    fn set_bypass(&mut self, bypass: bool) {
        self.bypassed = bypass;
    }

    fn is_bypassed(&self) -> bool {
        self.bypassed
    }

    fn reset(&mut self) {
        self.low_state_l = BiquadState::default();
        self.low_state_r = BiquadState::default();
        self.mid_state_l = BiquadState::default();
        self.mid_state_r = BiquadState::default();
        self.high_state_l = BiquadState::default();
        self.high_state_r = BiquadState::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_new() {
        let eq = EqModule::new(48000.0);
        assert_eq!(eq.low.freq_hz, 80.0);
        assert_eq!(eq.mid.freq_hz, 1000.0);
        assert_eq!(eq.high.freq_hz, 8000.0);
        assert!(!eq.is_bypassed());
    }

    #[test]
    fn test_set_low() {
        let mut eq = EqModule::new(48000.0);
        eq.set_low(100.0, 6.0, 1.5).unwrap();
        assert_eq!(eq.low.freq_hz, 100.0);
        assert_eq!(eq.low.gain_db, 6.0);
        assert_eq!(eq.low.q, 1.5);
    }

    #[test]
    fn test_set_low_invalid() {
        let mut eq = EqModule::new(48000.0);
        assert!(eq.set_low(10.0, 0.0, 1.0).is_err()); // Freq zu niedrig
        assert!(eq.set_low(100.0, 20.0, 1.0).is_err()); // Gain zu hoch
        assert!(eq.set_low(100.0, 0.0, 10.0).is_err()); // Q zu hoch
    }

    #[test]
    fn test_bypass() {
        let mut eq = EqModule::new(48000.0);
        eq.set_bypass(true);
        assert!(eq.is_bypassed());

        let mut buffer_l = vec![0.5; 256];
        let mut buffer_r = vec![0.5; 256];
        let expected = buffer_l.clone();

        eq.process(&mut buffer_l, &mut buffer_r);

        // Bei Bypass sollte Signal unverändert sein
        assert_eq!(buffer_l, expected);
        assert_eq!(buffer_r, expected);
    }

    #[test]
    fn test_reset() {
        let mut eq = EqModule::new(48000.0);
        let mut buffer_l = vec![0.5; 256];
        let mut buffer_r = vec![0.5; 256];
        eq.process(&mut buffer_l, &mut buffer_r);

        eq.reset();
        assert_eq!(eq.low_state_l.x1, 0.0);
        assert_eq!(eq.mid_state_r.y1, 0.0);
        assert_eq!(eq.high_state_l.x2, 0.0);
    }
}
