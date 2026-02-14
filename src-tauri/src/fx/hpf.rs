// Modul: fx/hpf — Hi-Pass Filter (Trittschall-Entfernung)
use super::{AudioEffect, FxType};

/// Hi-Pass Filter entfernt tiefe Frequenzen unter der Cutoff-Frequenz
#[derive(Debug)]
pub struct HiPassFilter {
    /// Cutoff-Frequenz in Hz (Standard: 80 Hz)
    pub cutoff_hz: f32,
    /// Filtersteilheit in dB/Oktave
    pub slope_db: f32,
    /// Effekt aktiv
    pub enabled: bool,
}

impl HiPassFilter {
    /// Neuen Hi-Pass Filter erstellen
    pub fn new() -> Self {
        // TODO: Standard-Werte setzen
        todo!("HiPassFilter::new")
    }
}

impl AudioEffect for HiPassFilter {
    fn name(&self) -> &str { "Hi-Pass Filter" }
    fn fx_type(&self) -> FxType { FxType::HiPassFilter }
    fn process(&mut self, _samples: &mut [f32]) { todo!("HiPassFilter::process") }
    fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }
    fn is_enabled(&self) -> bool { self.enabled }
}
