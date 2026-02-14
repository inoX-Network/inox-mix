// Modul: fx — FX-Chain Manager (Signal-Verarbeitung)

pub mod hpf;
pub mod denoise;
pub mod gate;
pub mod deesser;
pub mod eq;
pub mod compressor;
pub mod limiter;
pub mod autogain;

use serde::{Deserialize, Serialize};

/// Verfügbare FX-Typen in der Signalkette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FxType {
    HiPassFilter,
    Denoise,
    NoiseGate,
    DeEsser,
    Equalizer,
    Compressor,
    Limiter,
    AutoGain,
}

/// Trait den jedes FX-Modul implementieren muss
pub trait AudioEffect: Send + Sync {
    /// Effekt-Name für die UI
    fn name(&self) -> &str;
    /// Effekt-Typ
    fn fx_type(&self) -> FxType;
    /// Audio-Samples verarbeiten (in-place)
    fn process(&mut self, samples: &mut [f32]);
    /// Effekt aktivieren/deaktivieren
    fn set_enabled(&mut self, enabled: bool);
    /// Ist der Effekt aktiv?
    fn is_enabled(&self) -> bool;
}

/// FX-Chain verwaltet die geordnete Kette von Effekten pro Kanal
#[derive(Debug)]
pub struct FxChain {
    // TODO: Vec<Box<dyn AudioEffect>>
    // TODO: Bypass-State
}

impl FxChain {
    /// Neue leere FX-Chain erstellen
    pub fn new() -> Self {
        // TODO: Standard-Kette erstellen (HPF → Denoise → Gate → DeEsser → EQ → Comp → Limiter → AutoGain)
        todo!("FxChain::new")
    }

    /// Alle Effekte auf Audio-Buffer anwenden
    pub fn process(&mut self, _samples: &mut [f32]) {
        // TODO: Kette durchlaufen
        todo!("FxChain::process")
    }
}
