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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_fx_manager_new() {
        let mgr = VoiceFxManager::new();
        let state = mgr.get_state();
        assert_eq!(state.preset, VoiceFxPreset::None);
        assert!(!state.enabled);
        assert_eq!(state.dry_wet, 1.0);
    }

    #[test]
    fn test_set_preset() {
        let mut mgr = VoiceFxManager::new();
        mgr.set_preset(VoiceFxPreset::Robot);
        assert_eq!(mgr.get_state().preset, VoiceFxPreset::Robot);
    }

    #[test]
    fn test_set_enabled() {
        let mut mgr = VoiceFxManager::new();
        mgr.set_enabled(true);
        assert!(mgr.get_state().enabled);
        mgr.set_enabled(false);
        assert!(!mgr.get_state().enabled);
    }

    #[test]
    fn test_set_dry_wet() {
        let mut mgr = VoiceFxManager::new();
        mgr.set_dry_wet(0.5).unwrap();
        assert_eq!(mgr.get_state().dry_wet, 0.5);
    }

    #[test]
    fn test_set_dry_wet_invalid() {
        let mut mgr = VoiceFxManager::new();
        assert!(mgr.set_dry_wet(-0.1).is_err());
        assert!(mgr.set_dry_wet(1.5).is_err());
    }

    #[test]
    fn test_all_presets() {
        let presets = VoiceFxPreset::all();
        assert_eq!(presets.len(), 7);
        assert!(presets.contains(&VoiceFxPreset::None));
        assert!(presets.contains(&VoiceFxPreset::Robot));
        assert!(presets.contains(&VoiceFxPreset::Vader));
        assert!(presets.contains(&VoiceFxPreset::Chipmunk));
        assert!(presets.contains(&VoiceFxPreset::Megaphone));
        assert!(presets.contains(&VoiceFxPreset::Echo));
        assert!(presets.contains(&VoiceFxPreset::Radio));
    }

    #[test]
    fn test_preset_names() {
        assert_eq!(VoiceFxPreset::None.name(), "Aus");
        assert_eq!(VoiceFxPreset::Robot.name(), "Robot");
        assert_eq!(VoiceFxPreset::Vader.name(), "Vader");
        assert_eq!(VoiceFxPreset::Chipmunk.name(), "Chipmunk");
        assert_eq!(VoiceFxPreset::Megaphone.name(), "Megaphone");
        assert_eq!(VoiceFxPreset::Echo.name(), "Echo");
        assert_eq!(VoiceFxPreset::Radio.name(), "Radio");
    }
}
