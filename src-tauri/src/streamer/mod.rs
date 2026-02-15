// Modul: streamer — Streamer-spezifische Features (Ducking, Bleeper, Voice FX, Soundboard)

pub mod bleeper;
pub mod ducking;
pub mod ladspa_ffi;
pub mod ladspa_instance;
pub mod ladspa_loader;
pub mod soundboard;
pub mod voice_fx;
pub mod voice_fx_engine;

/// Streamer-Modul koordiniert alle Stream-bezogenen Funktionen
#[derive(Debug)]
pub struct StreamerEngine {
    // TODO: Ducking-State
    // TODO: Bleeper-State
    // TODO: Voice FX-State
    // TODO: Soundboard-State
}

impl StreamerEngine {
    /// Neue Streamer-Engine erstellen
    pub fn new() -> Self {
        // TODO: Alle Sub-Module initialisieren
        todo!("StreamerEngine::new")
    }
}
