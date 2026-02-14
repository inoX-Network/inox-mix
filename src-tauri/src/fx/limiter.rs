// Modul: fx/limiter — Brickwall Limiter (Clipping-Schutz)
use super::{AudioEffect, FxType};

/// Limiter verhindert Clipping durch harte Begrenzung
#[derive(Debug)]
pub struct Limiter {
    /// Decke in dB (Standard: -1.0)
    pub ceiling_db: f32,
    /// Release-Zeit in ms
    pub release_ms: f32,
    /// Effekt aktiv
    pub enabled: bool,
}

impl Limiter {
    /// Neuen Limiter erstellen
    pub fn new() -> Self {
        // TODO: Standard-Werte setzen
        todo!("Limiter::new")
    }
}

impl AudioEffect for Limiter {
    fn name(&self) -> &str { "Limiter" }
    fn fx_type(&self) -> FxType { FxType::Limiter }
    fn process(&mut self, _samples: &mut [f32]) { todo!("Limiter::process") }
    fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }
    fn is_enabled(&self) -> bool { self.enabled }
}
