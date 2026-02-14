// Modul: fx/gate — Noise Gate (Stumm bei Stille)
//
// Schaltet Audio unter Threshold stumm (reduziert Hintergrundgeräusche)
// SPEC: Threshold -60 bis 0 dB, Attack 0.1-50ms, Hold 0-500ms, Release 5-500ms

use super::{AudioProcessor, SAMPLE_RATE};

const MIN_THRESHOLD: f32 = -60.0;
const MAX_THRESHOLD: f32 = 0.0;
const DEFAULT_THRESHOLD: f32 = -40.0;

const MIN_ATTACK: f32 = 0.1;
const MAX_ATTACK: f32 = 50.0;
const DEFAULT_ATTACK: f32 = 5.0;

const MIN_HOLD: f32 = 0.0;
const MAX_HOLD: f32 = 500.0;
const DEFAULT_HOLD: f32 = 50.0;

const MIN_RELEASE: f32 = 5.0;
const MAX_RELEASE: f32 = 500.0;
const DEFAULT_RELEASE: f32 = 100.0;

/// Gate State
#[derive(Debug, Clone, Copy, PartialEq)]
enum GateState {
    Closed,  // Gate zu (stumm)
    Attack,  // Gate öffnet sich
    Open,    // Gate offen (Signal passiert)
    Hold,    // Gate hält offen (auch bei kurzen Pausen)
    Release, // Gate schließt sich
}

/// Noise Gate Modul
pub struct GateModule {
    /// Threshold in dB (-60 bis 0)
    threshold_db: f32,
    /// Attack in ms (0.1-50)
    attack_ms: f32,
    /// Hold in ms (0-500)
    hold_ms: f32,
    /// Release in ms (5-500)
    release_ms: f32,
    /// Bypass aktiv
    bypassed: bool,

    // Runtime State
    /// Aktueller Gate-State
    state: GateState,
    /// Envelope (0.0 = zu, 1.0 = offen)
    envelope: f32,
    /// Hold-Counter (in Samples)
    hold_samples: usize,
    hold_counter: usize,

    // Coefficients (berechnet aus ms-Werten)
    attack_coeff: f32,
    release_coeff: f32,
}

impl GateModule {
    /// Neues Gate mit Standard-Einstellungen
    pub fn new() -> Self {
        let mut gate = Self {
            threshold_db: DEFAULT_THRESHOLD,
            attack_ms: DEFAULT_ATTACK,
            hold_ms: DEFAULT_HOLD,
            release_ms: DEFAULT_RELEASE,
            bypassed: false,
            state: GateState::Closed,
            envelope: 0.0,
            hold_samples: 0,
            hold_counter: 0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
        };
        gate.update_coefficients();
        gate
    }

    /// Threshold setzen (-60 bis 0 dB)
    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold_db = threshold_db.clamp(MIN_THRESHOLD, MAX_THRESHOLD);
    }

    pub fn get_threshold(&self) -> f32 {
        self.threshold_db
    }

    /// Attack setzen (0.1-50 ms)
    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack_ms = attack_ms.clamp(MIN_ATTACK, MAX_ATTACK);
        self.update_coefficients();
    }

    pub fn get_attack(&self) -> f32 {
        self.attack_ms
    }

    /// Hold setzen (0-500 ms)
    pub fn set_hold(&mut self, hold_ms: f32) {
        self.hold_ms = hold_ms.clamp(MIN_HOLD, MAX_HOLD);
        self.update_coefficients();
    }

    pub fn get_hold(&self) -> f32 {
        self.hold_ms
    }

    /// Release setzen (5-500 ms)
    pub fn set_release(&mut self, release_ms: f32) {
        self.release_ms = release_ms.clamp(MIN_RELEASE, MAX_RELEASE);
        self.update_coefficients();
    }

    pub fn get_release(&self) -> f32 {
        self.release_ms
    }

    /// Coefficients aus ms-Werten berechnen
    fn update_coefficients(&mut self) {
        // Attack/Release als Exponential-Smooth-Factor
        // tau = time_ms / 1000.0 (in Sekunden)
        // coeff = 1.0 - exp(-1.0 / (tau * sample_rate))
        self.attack_coeff = 1.0 - (-1.0 / ((self.attack_ms / 1000.0) * SAMPLE_RATE)).exp();
        self.release_coeff = 1.0 - (-1.0 / ((self.release_ms / 1000.0) * SAMPLE_RATE)).exp();

        // Hold in Samples
        self.hold_samples = ((self.hold_ms / 1000.0) * SAMPLE_RATE) as usize;
    }

    /// Level in dB berechnen (RMS über L+R)
    #[inline]
    fn level_db(&self, sample_l: f32, sample_r: f32) -> f32 {
        let rms = ((sample_l * sample_l + sample_r * sample_r) / 2.0).sqrt();
        if rms > 0.0 {
            20.0 * rms.log10()
        } else {
            -100.0 // Sehr leise = -100 dB
        }
    }

    /// Gate State Update (pro Sample)
    fn update_state(&mut self, level_db: f32) {
        let above_threshold = level_db > self.threshold_db;

        match self.state {
            GateState::Closed => {
                if above_threshold {
                    self.state = GateState::Attack;
                }
            }
            GateState::Attack => {
                // Envelope steigt
                self.envelope += self.attack_coeff * (1.0 - self.envelope);
                if self.envelope >= 0.99 {
                    self.envelope = 1.0;
                    self.state = GateState::Open;
                }
            }
            GateState::Open => {
                if !above_threshold {
                    self.state = GateState::Hold;
                    self.hold_counter = self.hold_samples;
                }
            }
            GateState::Hold => {
                if above_threshold {
                    // Signal kam zurück
                    self.state = GateState::Open;
                } else if self.hold_counter > 0 {
                    self.hold_counter -= 1;
                } else {
                    // Hold abgelaufen
                    self.state = GateState::Release;
                }
            }
            GateState::Release => {
                if above_threshold {
                    // Signal kam zurück
                    self.state = GateState::Attack;
                } else {
                    // Envelope fällt
                    self.envelope -= self.release_coeff * self.envelope;
                    if self.envelope <= 0.01 {
                        self.envelope = 0.0;
                        self.state = GateState::Closed;
                    }
                }
            }
        }
    }
}

impl AudioProcessor for GateModule {
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        if self.bypassed {
            return;
        }

        for i in 0..buffer_l.len() {
            let level_db = self.level_db(buffer_l[i], buffer_r[i]);
            self.update_state(level_db);

            // Gain anwenden
            buffer_l[i] *= self.envelope;
            buffer_r[i] *= self.envelope;
        }
    }

    fn set_bypass(&mut self, bypass: bool) {
        self.bypassed = bypass;
    }

    fn is_bypassed(&self) -> bool {
        self.bypassed
    }

    fn reset(&mut self) {
        self.state = GateState::Closed;
        self.envelope = 0.0;
        self.hold_counter = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_new() {
        let gate = GateModule::new();
        assert_eq!(gate.threshold_db, DEFAULT_THRESHOLD);
        assert_eq!(gate.attack_ms, DEFAULT_ATTACK);
        assert_eq!(gate.hold_ms, DEFAULT_HOLD);
        assert_eq!(gate.release_ms, DEFAULT_RELEASE);
        assert!(!gate.bypassed);
    }

    #[test]
    fn test_gate_set_params() {
        let mut gate = GateModule::new();
        gate.set_threshold(-30.0);
        gate.set_attack(10.0);
        gate.set_hold(100.0);
        gate.set_release(200.0);

        assert_eq!(gate.get_threshold(), -30.0);
        assert_eq!(gate.get_attack(), 10.0);
        assert_eq!(gate.get_hold(), 100.0);
        assert_eq!(gate.get_release(), 200.0);
    }

    #[test]
    fn test_gate_param_clamp() {
        let mut gate = GateModule::new();

        // Threshold
        gate.set_threshold(-100.0);
        assert_eq!(gate.threshold_db, MIN_THRESHOLD);
        gate.set_threshold(10.0);
        assert_eq!(gate.threshold_db, MAX_THRESHOLD);

        // Attack
        gate.set_attack(0.01);
        assert_eq!(gate.attack_ms, MIN_ATTACK);
        gate.set_attack(100.0);
        assert_eq!(gate.attack_ms, MAX_ATTACK);
    }

    #[test]
    fn test_gate_bypass() {
        let mut gate = GateModule::new();
        gate.set_bypass(true);

        let mut buffer_l = vec![0.5; 256];
        let mut buffer_r = vec![0.5; 256];
        let expected = buffer_l.clone();

        gate.process(&mut buffer_l, &mut buffer_r);

        assert_eq!(buffer_l, expected);
        assert_eq!(buffer_r, expected);
    }

    #[test]
    fn test_gate_attenuates_below_threshold() {
        let mut gate = GateModule::new();
        gate.set_threshold(-20.0); // -20 dB Threshold

        // Leises Signal (-40 dB ≈ 0.01 Amplitude)
        let mut buffer_l = vec![0.01; 256];
        let mut buffer_r = vec![0.01; 256];

        gate.process(&mut buffer_l, &mut buffer_r);

        // Signal sollte reduziert/stumm sein
        let output_max = buffer_l.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        assert!(output_max < 0.005, "Gate sollte leises Signal dämpfen");
    }

    #[test]
    fn test_gate_passes_above_threshold() {
        let mut gate = GateModule::new();
        gate.set_threshold(-20.0);

        // Lautes Signal (-10 dB ≈ 0.316 Amplitude)
        let mut buffer_l = vec![0.316; 256];
        let mut buffer_r = vec![0.316; 256];

        gate.process(&mut buffer_l, &mut buffer_r);

        // Signal sollte durchkommen (Envelope wird auf 1.0 steigen)
        // Nach 256 Samples mit Attack 5ms sollte Gate weitgehend offen sein
        let output_avg: f32 = buffer_l.iter().sum::<f32>() / buffer_l.len() as f32;
        assert!(output_avg > 0.1, "Gate sollte lautes Signal passieren lassen");
    }

    #[test]
    fn test_gate_reset() {
        let mut gate = GateModule::new();
        gate.envelope = 0.5;
        gate.state = GateState::Open;

        gate.reset();

        assert_eq!(gate.envelope, 0.0);
        assert_eq!(gate.state, GateState::Closed);
        assert_eq!(gate.hold_counter, 0);
    }
}
