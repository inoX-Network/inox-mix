// Modul: fx — FX-Chain und Audio-Prozessoren
//
// 8-stufige Signal-Chain pro Input-Strip (SPEC: 03-signal-chain)
// Reihenfolge: HPF → AI-Denoise → Gate → De-Esser → EQ → Compressor → Limiter → Auto-Gain
//
// Phase 1: HPF + Gate implementiert
// Phase 2: Weitere 6 Module (TODO)

// Sub-Module (Phase 1: nur HPF und Gate implementiert)
pub mod hpf;
pub mod gate;

// Phase 2 Module (noch nicht implementiert)
// pub mod denoise;
// pub mod deesser;
// pub mod eq;
// pub mod compressor;
// pub mod limiter;
// pub mod autogain;

use serde::{Deserialize, Serialize};

// Audio-Konstanten
pub const SAMPLE_RATE: f32 = 48000.0;
pub const BUFFER_SIZE: usize = 256;

/// Audio-Prozessor Trait (Stereo L+R)
/// Alle FX-Module implementieren dieses Interface
pub trait AudioProcessor {
    /// Audio verarbeiten (L+R Channels, In-Place)
    fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]);

    /// Bypass-Status setzen
    fn set_bypass(&mut self, bypass: bool);

    /// Bypass-Status abfragen
    fn is_bypassed(&self) -> bool;

    /// Prozessor zurücksetzen (State löschen)
    fn reset(&mut self);
}

/// FX-Modul Typen (alle 8 laut SPEC)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FxModuleType {
    /// 1. Hi-Pass Filter (20-300 Hz, Standard 80 Hz)
    Hpf,
    /// 2. AI Denoise (DeepFilterNet/RNNoise, 0-100%)
    Denoise,
    /// 3. Noise Gate (Threshold, Attack, Hold, Release)
    Gate,
    /// 4. De-Esser (2-10 kHz, Reduction 0-20 dB)
    DeEsser,
    /// 5. Equalizer (3-Band: Low/Mid/High, je ±12 dB)
    Eq,
    /// 6. Compressor (Threshold, Ratio, Attack, Release)
    Compressor,
    /// 7. Limiter (Ceiling, Release)
    Limiter,
    /// 8. Auto-Gain (Target LUFS, Standard -14)
    AutoGain,
}

impl FxModuleType {
    /// Alle Module in Chain-Reihenfolge
    pub fn all() -> Vec<Self> {
        vec![
            Self::Hpf,
            Self::Denoise,
            Self::Gate,
            Self::DeEsser,
            Self::Eq,
            Self::Compressor,
            Self::Limiter,
            Self::AutoGain,
        ]
    }

    /// Modul-Name (für UI)
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hpf => "HPF",
            Self::Denoise => "AI-DN",
            Self::Gate => "GATE",
            Self::DeEsser => "DE-S",
            Self::Eq => "EQ",
            Self::Compressor => "COMP",
            Self::Limiter => "LIM",
            Self::AutoGain => "A-G",
        }
    }

    /// Modul-Farbe (abwechselnd Cyan/Orange)
    pub fn color(&self) -> &'static str {
        match self {
            Self::Hpf | Self::Gate | Self::Eq | Self::Limiter => "cyan",
            Self::Denoise | Self::DeEsser | Self::Compressor | Self::AutoGain => "orange",
        }
    }
}

/// FX-Modul Info (für Frontend)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FxModuleInfo {
    /// Modul-Typ
    pub module_type: FxModuleType,
    /// Enabled (nicht bypassed)
    pub enabled: bool,
    /// Parameter (Name → Wert)
    pub params: Vec<(String, f32)>,
}

/// FX-Chain (8 Module in fester Reihenfolge)
pub struct FxChain {
    // Phase 1: Nur HPF und Gate implementiert
    hpf: hpf::HpfModule,
    gate: gate::GateModule,

    // Phase 2: Weitere Module (noch nicht implementiert)
    // denoise: denoise::DenoiseModule,
    // deesser: deesser::DeEsserModule,
    // eq: eq::EqModule,
    // compressor: compressor::CompressorModule,
    // limiter: limiter::LimiterModule,
    // autogain: autogain::AutoGainModule,
}

impl FxChain {
    /// Neue FX-Chain mit Default-Settings
    pub fn new() -> Self {
        log::info!("FxChain::new() — Erstelle 8-stufige Signal-Chain (Phase 1: HPF + Gate)");
        Self {
            hpf: hpf::HpfModule::new(),
            gate: gate::GateModule::new(),
        }
    }

    /// Audio durch komplette Chain verarbeiten
    pub fn process(&mut self, buffer_l: &mut [f32], buffer_r: &mut [f32]) {
        // Chain-Reihenfolge (SPEC: strikt einhalten!)
        // 1. HPF
        self.hpf.process(buffer_l, buffer_r);

        // 2. AI Denoise (Phase 2)
        // self.denoise.process(buffer_l, buffer_r);

        // 3. Gate
        self.gate.process(buffer_l, buffer_r);

        // 4. De-Esser (Phase 2)
        // self.deesser.process(buffer_l, buffer_r);

        // 5. EQ (Phase 2)
        // self.eq.process(buffer_l, buffer_r);

        // 6. Compressor (Phase 2)
        // self.compressor.process(buffer_l, buffer_r);

        // 7. Limiter (Phase 2)
        // self.limiter.process(buffer_l, buffer_r);

        // 8. Auto-Gain (Phase 2)
        // self.autogain.process(buffer_l, buffer_r);
    }

    /// Modul-Info für Frontend
    pub fn get_module_info(&self, module_type: FxModuleType) -> Option<FxModuleInfo> {
        match module_type {
            FxModuleType::Hpf => Some(FxModuleInfo {
                module_type,
                enabled: !self.hpf.is_bypassed(),
                params: vec![("freq".to_string(), self.hpf.get_freq())],
            }),
            FxModuleType::Gate => Some(FxModuleInfo {
                module_type,
                enabled: !self.gate.is_bypassed(),
                params: vec![
                    ("threshold".to_string(), self.gate.get_threshold()),
                    ("attack".to_string(), self.gate.get_attack()),
                    ("hold".to_string(), self.gate.get_hold()),
                    ("release".to_string(), self.gate.get_release()),
                ],
            }),
            // Phase 2: Weitere Module
            _ => None,
        }
    }

    /// Alle Module als Info-Liste
    pub fn get_all_modules(&self) -> Vec<FxModuleInfo> {
        FxModuleType::all()
            .iter()
            .filter_map(|&t| self.get_module_info(t))
            .collect()
    }

    /// Parameter setzen
    pub fn set_param(&mut self, module_type: FxModuleType, param_name: &str, value: f32) -> Result<(), String> {
        match module_type {
            FxModuleType::Hpf => {
                if param_name == "freq" {
                    self.hpf.set_freq(value);
                    Ok(())
                } else {
                    Err(format!("Unbekannter Parameter: {}", param_name))
                }
            }
            FxModuleType::Gate => {
                match param_name {
                    "threshold" => self.gate.set_threshold(value),
                    "attack" => self.gate.set_attack(value),
                    "hold" => self.gate.set_hold(value),
                    "release" => self.gate.set_release(value),
                    _ => return Err(format!("Unbekannter Parameter: {}", param_name)),
                }
                Ok(())
            }
            // Phase 2: Weitere Module
            _ => Err(format!("Modul {:?} noch nicht implementiert", module_type)),
        }
    }

    /// Bypass setzen
    pub fn set_bypass(&mut self, module_type: FxModuleType, bypass: bool) -> Result<(), String> {
        match module_type {
            FxModuleType::Hpf => {
                self.hpf.set_bypass(bypass);
                Ok(())
            }
            FxModuleType::Gate => {
                self.gate.set_bypass(bypass);
                Ok(())
            }
            // Phase 2: Weitere Module
            _ => Err(format!("Modul {:?} noch nicht implementiert", module_type)),
        }
    }

    /// Chain zurücksetzen (alle Module)
    pub fn reset(&mut self) {
        self.hpf.reset();
        self.gate.reset();
        // Phase 2: Weitere Module
    }
}

impl Default for FxChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fx_module_type_all() {
        let modules = FxModuleType::all();
        assert_eq!(modules.len(), 8);
        assert_eq!(modules[0], FxModuleType::Hpf);
        assert_eq!(modules[7], FxModuleType::AutoGain);
    }

    #[test]
    fn test_fx_module_names() {
        assert_eq!(FxModuleType::Hpf.name(), "HPF");
        assert_eq!(FxModuleType::Gate.name(), "GATE");
        assert_eq!(FxModuleType::Eq.name(), "EQ");
    }

    #[test]
    fn test_fx_module_colors() {
        // Abwechselnd Cyan/Orange
        assert_eq!(FxModuleType::Hpf.color(), "cyan");
        assert_eq!(FxModuleType::Denoise.color(), "orange");
        assert_eq!(FxModuleType::Gate.color(), "cyan");
        assert_eq!(FxModuleType::DeEsser.color(), "orange");
    }

    #[test]
    fn test_fx_chain_new() {
        let chain = FxChain::new();
        // HPF und Gate sollten standardmäßig enabled sein
        assert!(!chain.hpf.is_bypassed());
        assert!(!chain.gate.is_bypassed());
    }

    #[test]
    fn test_fx_chain_process_passthrough() {
        let mut chain = FxChain::new();
        // Alle Module bypassen
        chain.set_bypass(FxModuleType::Hpf, true).unwrap();
        chain.set_bypass(FxModuleType::Gate, true).unwrap();

        let mut buffer_l = vec![0.5; 256];
        let mut buffer_r = vec![0.5; 256];
        let expected = buffer_l.clone();

        chain.process(&mut buffer_l, &mut buffer_r);

        // Bei Bypass sollte Signal unverändert sein
        assert_eq!(buffer_l, expected);
        assert_eq!(buffer_r, expected);
    }

    #[test]
    fn test_get_all_modules() {
        let chain = FxChain::new();
        let modules = chain.get_all_modules();
        // Phase 1: Nur HPF und Gate
        assert_eq!(modules.len(), 2);
        assert_eq!(modules[0].module_type, FxModuleType::Hpf);
        assert_eq!(modules[1].module_type, FxModuleType::Gate);
    }
}
