// Modul: streamer/voice_fx_engine — Voice FX Engine mit LADSPA-Integration
//
// Lädt LADSPA-Plugins für verschiedene Voice-Presets
use super::ladspa_instance::LadspaInstance;
use super::ladspa_loader::LadspaLoader;
use super::voice_fx::VoiceFxPreset;
use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Plugin-Chain für ein Voice-Preset
struct PluginChain {
    instances: Vec<LadspaInstance>,
    preset: VoiceFxPreset,
}

/// Voice FX Engine mit LADSPA
pub struct VoiceFxEngine {
    loader: LadspaLoader,
    active_chain: Option<PluginChain>,
    sample_rate: u64,
    enabled: bool,
}

impl VoiceFxEngine {
    /// Neue Voice FX Engine erstellen
    pub fn new(sample_rate: u64) -> Self {
        let mut loader = LadspaLoader::new();

        // LADSPA-Plugins scannen
        match loader.scan_plugins() {
            Ok(count) => {
                info!("✓ Voice FX Engine: {} LADSPA-Plugins gefunden", count);
                if count > 0 {
                    info!("Verfügbare Plugins:");
                    for label in loader.list_plugins().iter().take(10) {
                        info!("  - {}", label);
                    }
                    if loader.plugin_count() > 10 {
                        info!("  ... und {} weitere", loader.plugin_count() - 10);
                    }
                }
            }
            Err(e) => {
                warn!("⚠ LADSPA-Scan fehlgeschlagen: {}", e);
                warn!("  Voice FX werden ohne LADSPA-Plugins laufen (Bypass-Modus)");
                warn!("  Installiere Plugins: sudo apt install swh-plugins tap-plugins cmt");
            }
        }

        Self {
            loader,
            active_chain: None,
            sample_rate,
            enabled: false,
        }
    }

    /// Preset aktivieren und Plugin-Chain laden
    pub fn set_preset(&mut self, preset: VoiceFxPreset) -> Result<(), String> {
        info!("Voice FX Preset wechseln: {:?}", preset);

        // Alte Chain deaktivieren
        self.active_chain = None;

        if preset == VoiceFxPreset::None {
            return Ok(());
        }

        // Plugin-Chain für Preset erstellen
        let chain = self.create_preset_chain(preset)?;
        self.active_chain = Some(chain);

        Ok(())
    }

    /// Plugin-Chain für Preset erstellen
    fn create_preset_chain(&mut self, preset: VoiceFxPreset) -> Result<PluginChain, String> {
        let mut instances = Vec::new();

        // Preset-spezifische Plugin-Konfiguration
        match preset {
            VoiceFxPreset::Robot => {
                // Robot: Ring-Modulator + Pitch-Quantisierung
                if let Some(plugin) = self.find_plugin(&["ringmod", "ring_modulator"]) {
                    let instance = LadspaInstance::new(plugin.descriptor_ptr, self.sample_rate)?;
                    instances.push(instance);
                    info!("  ✓ Robot: Ring-Modulator geladen");
                } else {
                    warn!("  ⚠ Robot: Kein Ring-Modulator gefunden");
                }
            }

            VoiceFxPreset::Vader => {
                // Vader: Pitch-Shifter (down) + Reverb
                if let Some(plugin) = self.find_plugin(&["pitch", "pitchscale"]) {
                    let mut instance = LadspaInstance::new(plugin.descriptor_ptr, self.sample_rate)?;
                    // TODO: Pitch runter setzen (z.B. -5 Halbtöne)
                    instances.push(instance);
                    info!("  ✓ Vader: Pitch-Shifter geladen");
                } else {
                    warn!("  ⚠ Vader: Kein Pitch-Shifter gefunden");
                }
            }

            VoiceFxPreset::Chipmunk => {
                // Chipmunk: Pitch-Shifter (up)
                if let Some(plugin) = self.find_plugin(&["pitch", "pitchscale"]) {
                    let mut instance = LadspaInstance::new(plugin.descriptor_ptr, self.sample_rate)?;
                    // TODO: Pitch hoch setzen (z.B. +7 Halbtöne)
                    instances.push(instance);
                    info!("  ✓ Chipmunk: Pitch-Shifter geladen");
                } else {
                    warn!("  ⚠ Chipmunk: Kein Pitch-Shifter gefunden");
                }
            }

            VoiceFxPreset::Megaphone => {
                // Megaphone: Bandpass + leichte Verzerrung
                if let Some(plugin) = self.find_plugin(&["bandpass", "bpf"]) {
                    let instance = LadspaInstance::new(plugin.descriptor_ptr, self.sample_rate)?;
                    instances.push(instance);
                    info!("  ✓ Megaphone: Bandpass geladen");
                } else {
                    warn!("  ⚠ Megaphone: Kein Bandpass gefunden");
                }
            }

            VoiceFxPreset::Echo => {
                // Echo: Delay + Reverb
                if let Some(plugin) = self.find_plugin(&["delay", "echo"]) {
                    let instance = LadspaInstance::new(plugin.descriptor_ptr, self.sample_rate)?;
                    instances.push(instance);
                    info!("  ✓ Echo: Delay geladen");
                } else {
                    warn!("  ⚠ Echo: Kein Delay gefunden");
                }
            }

            VoiceFxPreset::Radio => {
                // Radio: Bandpass + Kompression
                if let Some(plugin) = self.find_plugin(&["bandpass", "bpf"]) {
                    let instance = LadspaInstance::new(plugin.descriptor_ptr, self.sample_rate)?;
                    instances.push(instance);
                    info!("  ✓ Radio: Bandpass geladen");
                } else {
                    warn!("  ⚠ Radio: Kein Bandpass gefunden");
                }
            }

            VoiceFxPreset::None => {}
        }

        if instances.is_empty() {
            return Err(format!("Keine Plugins für Preset {:?} gefunden", preset));
        }

        Ok(PluginChain { instances, preset })
    }

    /// Plugin nach Label-Substring finden
    fn find_plugin(&self, search_terms: &[&str]) -> Option<&super::ladspa_loader::LoadedPlugin> {
        for term in search_terms {
            for label in self.loader.list_plugins() {
                if label.to_lowercase().contains(&term.to_lowercase()) {
                    return self.loader.find_plugin(&label);
                }
            }
        }
        None
    }

    /// Audio verarbeiten
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<(), String> {
        // Bypass wenn disabled oder keine Chain
        if !self.enabled || self.active_chain.is_none() {
            output.copy_from_slice(input);
            return Ok(());
        }

        let chain = self.active_chain.as_mut().unwrap();

        // Audio durch Plugin-Chain schicken
        let mut buffer_a = input.to_vec();
        let mut buffer_b = vec![0.0f32; input.len()];

        for (i, instance) in chain.instances.iter_mut().enumerate() {
            if i % 2 == 0 {
                instance.process_mono(&buffer_a, &mut buffer_b)?;
            } else {
                instance.process_mono(&buffer_b, &mut buffer_a)?;
            }
        }

        // Ergebnis kopieren (je nachdem ob gerade oder ungerade Anzahl Plugins)
        if chain.instances.len() % 2 == 0 {
            output.copy_from_slice(&buffer_a);
        } else {
            output.copy_from_slice(&buffer_b);
        }

        Ok(())
    }

    /// Voice FX aktivieren/deaktivieren
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        info!("Voice FX: {}", if enabled { "Aktiviert" } else { "Deaktiviert" });
    }

    /// Ist aktiviert?
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Anzahl verfügbarer Plugins
    pub fn plugin_count(&self) -> usize {
        self.loader.plugin_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_fx_engine_creation() {
        let engine = VoiceFxEngine::new(48000);
        println!("Voice FX Engine erstellt mit {} Plugins", engine.plugin_count());
    }

    #[test]
    fn test_preset_loading() {
        let mut engine = VoiceFxEngine::new(48000);

        if engine.plugin_count() > 0 {
            match engine.set_preset(VoiceFxPreset::Robot) {
                Ok(_) => println!("✓ Robot-Preset geladen"),
                Err(e) => println!("⚠ Robot-Preset Fehler: {}", e),
            }
        } else {
            println!("⚠ Keine LADSPA-Plugins für Test verfügbar");
        }
    }

    #[test]
    fn test_audio_processing() {
        let mut engine = VoiceFxEngine::new(48000);
        engine.set_enabled(true);

        let input = vec![0.5f32; 1024];
        let mut output = vec![0.0f32; 1024];

        match engine.process(&input, &mut output) {
            Ok(_) => println!("✓ Audio-Processing erfolgreich"),
            Err(e) => println!("⚠ Processing-Fehler: {}", e),
        }
    }
}
