// Modul: streamer/voice_fx — Stimm-Effekte (Robot, Vader, Chipmunk, etc.)
//
// Phase 1: State Management und Struktur
// TODO Phase 2: PipeWire Filter-Chain Integration mit LADSPA/LV2 Plugins
// SPEC: 08-voice-fx

use serde::{Deserialize, Serialize};

/// Voice FX Presets (Stimm-Effekte)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VoiceFxPreset {
    /// Kein Effekt (Bypass)
    None,
    /// Roboter-Stimme (Pitch-Quantisierung + Ring-Modulator)
    Robot,
    /// Tiefe Stimme (Pitch runter + Formant-Shift + Hall)
    Vader,
    /// Hohe Stimme (Pitch hoch + Formant-Shift)
    Chipmunk,
    /// Megaphone (Bandpass + Verzerrung)
    Megaphone,
    /// Echo/Hall (Hall + Delay)
    Echo,
    /// Radio (Bandpass + Kompression + Rauschen)
    Radio,
}

impl VoiceFxPreset {
    /// Alle verfügbaren Presets
    pub fn all() -> Vec<Self> {
        vec![
            Self::None,
            Self::Robot,
            Self::Vader,
            Self::Chipmunk,
            Self::Megaphone,
            Self::Echo,
            Self::Radio,
        ]
    }

    /// Preset-Name für UI
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "Aus",
            Self::Robot => "Robot",
            Self::Vader => "Vader",
            Self::Chipmunk => "Chipmunk",
            Self::Megaphone => "Megaphone",
            Self::Echo => "Echo",
            Self::Radio => "Radio",
        }
    }
}

/// Voice FX State (für Frontend)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceFxState {
    /// Aktives Preset
    pub preset: VoiceFxPreset,
    /// FX enabled (Master-Toggle)
    pub enabled: bool,
    /// Dry/Wet Mix (0.0 = Original, 1.0 = 100% FX)
    pub dry_wet: f32,
}

impl Default for VoiceFxState {
    fn default() -> Self {
        Self {
            preset: VoiceFxPreset::None,
            enabled: false,
            dry_wet: 1.0,
        }
    }
}

/// Voice FX Manager (State Management)
///
/// Phase 1: Nur State-Verwaltung
/// TODO Phase 2: PipeWire Filter-Chain mit LADSPA/LV2
pub struct VoiceFxManager {
    state: VoiceFxState,
}

impl VoiceFxManager {
    /// Neuen Voice FX Manager erstellen
    pub fn new() -> Self {
        Self {
            state: VoiceFxState::default(),
        }
    }

    /// Preset setzen
    pub fn set_preset(&mut self, preset: VoiceFxPreset) {
        self.state.preset = preset;
        log::info!("Voice FX Preset: {:?}", preset);
        // TODO: PipeWire Filter-Chain konfigurieren
    }

    /// Enabled setzen (Master-Toggle)
    pub fn set_enabled(&mut self, enabled: bool) {
        self.state.enabled = enabled;
        log::info!("Voice FX Enabled: {}", enabled);
        // TODO: PipeWire Filter-Chain aktivieren/deaktivieren
    }

    /// Dry/Wet Mix setzen (0.0-1.0)
    pub fn set_dry_wet(&mut self, dry_wet: f32) -> Result<(), String> {
        if !(0.0..=1.0).contains(&dry_wet) {
            return Err(format!("Dry/Wet außerhalb: {} (0.0-1.0)", dry_wet));
        }
        self.state.dry_wet = dry_wet;
        log::info!("Voice FX Dry/Wet: {:.0}%", dry_wet * 100.0);
        // TODO: Mix-Regler in PipeWire Filter-Chain setzen
        Ok(())
    }

    /// Aktuellen State abrufen
    pub fn get_state(&self) -> &VoiceFxState {
        &self.state
    }
}

impl Default for VoiceFxManager {
    fn default() -> Self {
        Self::new()
    }
}
