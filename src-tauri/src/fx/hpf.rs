// Modul: fx/hpf — Hi-Pass Filter (Butterworth 2nd-Order)
//
// Entfernt tiefe Frequenzen unter Cutoff (z.B. Trittschall, Rumpeln)
// SPEC: 20-300 Hz, Standard 80 Hz

use super::{AudioProcessor, SAMPLE_RATE};

const MIN_FREQ: f32 = 20.0;
const MAX_FREQ: f32 = 300.0;
const DEFAULT_FREQ: f32 = 80.0;

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

/// Hi-Pass Filter (Butterworth 2nd-Order)
pub struct HpfModule {
    /// Cutoff-Frequenz in Hz (20-300 Hz)
    freq: f32,
    /// Bypass aktiv
    bypassed: bool,
    /// Biquad-Koeffizienten
    a0: f32,
    a1: f32,
    a2: f32,
    b1: f32,
    b2: f32,
    /// Filter State (L+R)
    state_l: BiquadState,
    state_r: BiquadState,
}

impl HpfModule {
    /// Neuen HPF mit Standard-Einstellungen (80 Hz)
    pub fn new() -> Self {
        let mut hpf = Self {
            freq: DEFAULT_FREQ,
            bypassed: false,
            a0: 1.0,
            a1: 0.0,
            a2: 0.0,
            b1: 0.0,
            b2: 0.0,
            state_l: BiquadState::default(),
            state_r: BiquadState::default(),
        };
        hpf.update_coefficients();
        hpf
    }

    /// Cutoff-Frequenz setzen (20-300 Hz)
    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq.clamp(MIN_FREQ, MAX_FREQ);
        self.update_coefficients();
    }

    /// Cutoff-Frequenz abfragen
    pub fn get_freq(&self) -> f32 {
        self.freq
    }

    /// Biquad-Koeffizienten berechnen (Butterworth HPF)
    fn update_coefficients(&mut self) {
        let omega = 2.0 * std::f32::consts::PI * self.freq / SAMPLE_RATE;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * 0.7071); // Q = 0.7071 (Butterworth)

        let b0 = (1.0 + cs) / 2.0;
        let b1_raw = -(1.0 + cs);
        let b2 = (1.0 + cs) / 2.0;
        let a0_raw = 1.0 + alpha;
        let a1_raw = -2.0 * cs;
        let a2_raw = 1.0 - alpha;

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

impl AudioProcessor for HpfModule {
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        if self.bypassed {
            return;
        }

        for i in 0..buffer_l.len() {
            buffer_l[i] = self.process_sample(buffer_l[i], &mut self.state_l);
            buffer_r[i] = self.process_sample(buffer_r[i], &mut self.state_r);
        }
    }

    fn set_bypass(&mut self, bypass: bool) {
        self.bypassed = bypass;
    }

    fn is_bypassed(&self) -> bool {
        self.bypassed
    }

    fn reset(&mut self) {
        self.state_l = BiquadState::default();
        self.state_r = BiquadState::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hpf_new() {
        let hpf = HpfModule::new();
        assert_eq!(hpf.freq, DEFAULT_FREQ);
        assert!(!hpf.bypassed);
    }

    #[test]
    fn test_hpf_set_freq() {
        let mut hpf = HpfModule::new();
        hpf.set_freq(100.0);
        assert_eq!(hpf.freq, 100.0);
    }

    #[test]
    fn test_hpf_freq_clamp() {
        let mut hpf = HpfModule::new();
        // Unter Minimum
        hpf.set_freq(10.0);
        assert_eq!(hpf.freq, MIN_FREQ);
        // Über Maximum
        hpf.set_freq(500.0);
        assert_eq!(hpf.freq, MAX_FREQ);
    }

    #[test]
    fn test_hpf_bypass() {
        let mut hpf = HpfModule::new();
        hpf.set_bypass(true);
        assert!(hpf.is_bypassed());

        let mut buffer_l = vec![0.5; 256];
        let mut buffer_r = vec![0.5; 256];
        let expected = buffer_l.clone();

        hpf.process(&mut buffer_l, &mut buffer_r);

        // Bei Bypass sollte Signal unverändert sein
        assert_eq!(buffer_l, expected);
        assert_eq!(buffer_r, expected);
    }

    #[test]
    fn test_hpf_attenuates_low_freq() {
        let mut hpf = HpfModule::new();
        hpf.set_freq(100.0);

        // Generiere 20 Hz Sinus (unter Cutoff)
        let freq = 20.0;
        let mut buffer_l: Vec<f32> = (0..256)
            .map(|i| (2.0 * std::f32::consts::PI * freq * i as f32 / SAMPLE_RATE).sin())
            .collect();
        let mut buffer_r = buffer_l.clone();

        let input_rms = rms(&buffer_l);
        hpf.process(&mut buffer_l, &mut buffer_r);
        let output_rms = rms(&buffer_l);

        // Signal unter Cutoff sollte gedämpft sein
        assert!(output_rms < input_rms * 0.5, "HPF sollte tiefe Frequenzen dämpfen");
    }

    #[test]
    fn test_hpf_passes_high_freq() {
        let mut hpf = HpfModule::new();
        hpf.set_freq(80.0);

        // Generiere 1000 Hz Sinus (über Cutoff)
        let freq = 1000.0;
        let mut buffer_l: Vec<f32> = (0..256)
            .map(|i| (2.0 * std::f32::consts::PI * freq * i as f32 / SAMPLE_RATE).sin())
            .collect();
        let mut buffer_r = buffer_l.clone();

        let input_rms = rms(&buffer_l);
        hpf.process(&mut buffer_l, &mut buffer_r);
        let output_rms = rms(&buffer_l);

        // Signal über Cutoff sollte weitgehend unverändert sein
        assert!((output_rms - input_rms).abs() < 0.1, "HPF sollte hohe Frequenzen passieren lassen");
    }

    #[test]
    fn test_hpf_reset() {
        let mut hpf = HpfModule::new();
        // Verarbeite etwas Audio (setzt State)
        let mut buffer_l = vec![0.5; 256];
        let mut buffer_r = vec![0.5; 256];
        hpf.process(&mut buffer_l, &mut buffer_r);

        // Reset
        hpf.reset();
        assert_eq!(hpf.state_l.x1, 0.0);
        assert_eq!(hpf.state_l.y1, 0.0);
        assert_eq!(hpf.state_r.x1, 0.0);
        assert_eq!(hpf.state_r.y1, 0.0);
    }

    // Hilfsfunktion: RMS berechnen
    fn rms(buffer: &[f32]) -> f32 {
        let sum_sq: f32 = buffer.iter().map(|&x| x * x).sum();
        (sum_sq / buffer.len() as f32).sqrt()
    }
}
