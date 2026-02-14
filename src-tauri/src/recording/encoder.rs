// Modul: recording/encoder — Audio-Encoder (FLAC, WAV, OGG)
//
// Encoder für verschiedene Audio-Formate mit hound crate
// SPEC: 11-recording

use hound::{WavWriter, WavSpec, SampleFormat};
use std::path::PathBuf;

/// Audio-Encoder Trait für verschiedene Formate
pub trait AudioEncoder: Send {
    /// Encoder-Name
    fn name(&self) -> &str;
    /// Audio-Samples zum Encoder schreiben (Stereo interleaved: L, R, L, R, ...)
    fn write_samples(&mut self, samples: &[f32]) -> Result<(), String>;
    /// Encoding finalisieren und Datei schließen
    fn finalize(&mut self) -> Result<(), String>;
}

/// WAV-Encoder (unkomprimiert, 48kHz Stereo PCM)
pub struct WavEncoder {
    writer: Option<WavWriter<std::io::BufWriter<std::fs::File>>>,
    path: PathBuf,
}

impl WavEncoder {
    /// Neuen WAV-Encoder erstellen
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let spec = WavSpec {
            channels: 2,              // Stereo
            sample_rate: 48000,       // 48 kHz
            bits_per_sample: 32,      // 32-bit float
            sample_format: SampleFormat::Float,
        };

        let writer = WavWriter::create(&path, spec)
            .map_err(|e| format!("WAV-Encoder konnte Datei nicht erstellen: {}", e))?;

        Ok(Self {
            writer: Some(writer),
            path,
        })
    }

    /// Pfad zur Aufnahme-Datei
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl AudioEncoder for WavEncoder {
    fn name(&self) -> &str {
        "WAV"
    }

    fn write_samples(&mut self, samples: &[f32]) -> Result<(), String> {
        let writer = self.writer.as_mut()
            .ok_or_else(|| "WAV-Encoder bereits finalisiert".to_string())?;

        for &sample in samples {
            writer.write_sample(sample)
                .map_err(|e| format!("WAV-Encoder Schreibfehler: {}", e))?;
        }

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), String> {
        if let Some(writer) = self.writer.take() {
            writer.finalize()
                .map_err(|e| format!("WAV-Encoder Finalisierung fehlgeschlagen: {}", e))?;
        }
        Ok(())
    }
}

/// FLAC-Encoder (verlustfrei komprimiert, 48kHz Stereo)
/// HINWEIS: hound unterstützt nur WAV, für FLAC würde claxon oder flacenc benötigt
pub struct FlacEncoder {
    // Placeholder — FLAC-Support würde separates Crate erfordern (z.B. claxon)
    _path: PathBuf,
}

impl FlacEncoder {
    #[allow(dead_code)]
    pub fn new(path: PathBuf) -> Result<Self, String> {
        // FLAC-Encoding mit hound nicht möglich
        Err(format!("FLAC-Encoder noch nicht implementiert (benötigt claxon crate): {:?}", path))
    }
}

/// OGG Vorbis-Encoder (verlustbehaftet)
pub struct OggEncoder {
    _path: PathBuf,
}

impl OggEncoder {
    #[allow(dead_code)]
    pub fn new(path: PathBuf) -> Result<Self, String> {
        Err(format!("OGG-Encoder noch nicht implementiert: {:?}", path))
    }
}
