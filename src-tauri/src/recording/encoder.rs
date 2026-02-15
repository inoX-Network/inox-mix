// Modul: recording/encoder — Audio-Encoder (FLAC, WAV, OGG)
//
// Encoder für verschiedene Audio-Formate
// FLAC: Nutzt WAV-Zwischenspeicherung + flac CLI für Konvertierung
// SPEC: 11-recording

use hound::{SampleFormat, WavSpec, WavWriter};
use std::path::PathBuf;
use std::process::Command;

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
            channels: 2,         // Stereo
            sample_rate: 48000,  // 48 kHz
            bits_per_sample: 32, // 32-bit float
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
        let writer = self
            .writer
            .as_mut()
            .ok_or_else(|| "WAV-Encoder bereits finalisiert".to_string())?;

        for &sample in samples {
            writer
                .write_sample(sample)
                .map_err(|e| format!("WAV-Encoder Schreibfehler: {}", e))?;
        }

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), String> {
        if let Some(writer) = self.writer.take() {
            writer
                .finalize()
                .map_err(|e| format!("WAV-Encoder Finalisierung fehlgeschlagen: {}", e))?;
        }
        Ok(())
    }
}

/// FLAC-Encoder (verlustfrei komprimiert, 48kHz Stereo)
/// Strategie: WAV aufnehmen, dann via `flac` CLI zu FLAC konvertieren
pub struct FlacEncoder {
    wav_encoder: WavEncoder,
    flac_path: PathBuf,
}

impl FlacEncoder {
    /// Neuen FLAC-Encoder erstellen (48kHz Stereo)
    ///
    /// Nimmt zunächst als WAV auf, konvertiert bei finalize() zu FLAC
    pub fn new(path: PathBuf) -> Result<Self, String> {
        // Temporäre WAV-Datei (wird später zu FLAC konvertiert)
        let temp_wav = path.with_extension("tmp.wav");
        let wav_encoder = WavEncoder::new(temp_wav)?;

        Ok(Self {
            wav_encoder,
            flac_path: path,
        })
    }

    /// Pfad zur finalen FLAC-Datei
    pub fn path(&self) -> &PathBuf {
        &self.flac_path
    }
}

impl AudioEncoder for FlacEncoder {
    fn name(&self) -> &str {
        "FLAC"
    }

    fn write_samples(&mut self, samples: &[f32]) -> Result<(), String> {
        // Samples in temporäre WAV-Datei schreiben
        self.wav_encoder.write_samples(samples)
    }

    fn finalize(&mut self) -> Result<(), String> {
        // WAV-Encoder finalisieren
        self.wav_encoder.finalize()?;

        let temp_wav = self.wav_encoder.path();

        // FLAC-Konvertierung via CLI (falls `flac` installiert)
        log::info!(
            "Konvertiere WAV → FLAC: {:?} → {:?}",
            temp_wav,
            self.flac_path
        );

        let output = Command::new("flac")
            .arg("--silent") // Keine Fortschritts-Ausgabe
            .arg("--best") // Maximale Kompression
            .arg("--output-name")
            .arg(&self.flac_path)
            .arg(temp_wav)
            .output();

        match output {
            Ok(result) if result.status.success() => {
                // Temporäre WAV-Datei löschen
                if let Err(e) = std::fs::remove_file(temp_wav) {
                    log::warn!("Konnte temporäre WAV-Datei nicht löschen: {}", e);
                }
                log::info!("FLAC-Konvertierung erfolgreich: {:?}", self.flac_path);
                Ok(())
            }
            Ok(result) => {
                let stderr = String::from_utf8_lossy(&result.stderr);
                Err(format!("flac CLI fehlgeschlagen: {}", stderr))
            }
            Err(_) => {
                // Fallback: WAV-Datei umbenennen zu .wav (FLAC CLI nicht installiert)
                let fallback_path = self.flac_path.with_extension("wav");
                std::fs::rename(temp_wav, &fallback_path)
                    .map_err(|e| format!("Umbenennung fehlgeschlagen: {}", e))?;

                log::warn!(
                    "FLAC CLI nicht installiert, WAV-Datei gespeichert: {:?}",
                    fallback_path
                );
                Err("FLAC-Encoding fehlgeschlagen: flac CLI nicht installiert (Datei als WAV gespeichert)".to_string())
            }
        }
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
