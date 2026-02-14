// Modul: recording/encoder — Audio-Encoder (FLAC, WAV, OGG)

/// Audio-Encoder Trait für verschiedene Formate
pub trait AudioEncoder: Send {
    /// Encoder-Name
    fn name(&self) -> &str;
    /// Audio-Samples zum Encoder schreiben
    fn write_samples(&mut self, samples: &[f32]) -> Result<(), String>;
    /// Encoding finalisieren und Datei schließen
    fn finalize(&mut self) -> Result<(), String>;
}

/// WAV-Encoder (unkomprimiert)
#[derive(Debug)]
pub struct WavEncoder {
    // TODO: Datei-Handle
    // TODO: Header-Infos
}

/// FLAC-Encoder (verlustfrei)
#[derive(Debug)]
pub struct FlacEncoder {
    // TODO: FLAC-Stream
}

/// OGG Vorbis-Encoder (verlustbehaftet)
#[derive(Debug)]
pub struct OggEncoder {
    // TODO: Vorbis-Stream
}
