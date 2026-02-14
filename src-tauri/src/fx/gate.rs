// Modul: fx/gate — Noise Gate (Stumm bei Stille)
use super::{AudioEffect, FxType};

/// Noise Gate schaltet Audio unter dem Schwellwert stumm
#[derive(Debug)]
pub struct NoiseGate {
    /// Schwellwert in dB
    pub threshold_db: f32,
    /// Attack-Zeit in ms
    pub attack_ms: f32,
    /// Release-Zeit in ms
    pub release_ms: f32,
    /// Effekt aktiv
    pub enabled: bool,
}

impl NoiseGate {
    /// Neues Noise Gate erstellen
    pub fn new() -> Self {
        // TODO: Standard-Werte setzen
        todo!("NoiseGate::new")
    }
}

impl AudioEffect for NoiseGate {
    fn name(&self) -> &str { "Noise Gate" }
    fn fx_type(&self) -> FxType { FxType::NoiseGate }
    fn process(&mut self, _samples: &mut [f32]) { todo!("NoiseGate::process") }
    fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }
    fn is_enabled(&self) -> bool { self.enabled }
}
