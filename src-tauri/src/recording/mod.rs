// Modul: recording — Audio-Aufnahme Engine
//
// Verwaltet aktive Aufnahmen pro Source (Bus oder Strip)
// SPEC: 11-recording

pub mod encoder;

use encoder::{AudioEncoder, WavEncoder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Aufnahme-Format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordingFormat {
    /// WAV (unkomprimiert, höchste Qualität)
    Wav,
    /// FLAC (verlustfrei komprimiert) — noch nicht implementiert
    Flac,
    /// OGG Vorbis (verlustbehaftet) — noch nicht implementiert
    Ogg,
}

/// Info über abgeschlossene Aufnahme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingInfo {
    /// Dateipfad
    pub path: String,
    /// Dauer in Sekunden
    pub duration_secs: f32,
    /// Dateigröße in Bytes
    pub size_bytes: u64,
}

/// Status einer aktiven Aufnahme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRecording {
    /// Source-ID (Bus oder Strip)
    pub source_id: String,
    /// Format (wav, flac, ogg)
    pub format: RecordingFormat,
    /// Dateipfad
    pub path: String,
    /// Start-Zeitstempel (Unix Epoch Sekunden)
    pub start_time: u64,
    /// Anzahl geschriebene Samples
    pub samples_written: usize,
}

/// Interne Aufnahme mit Encoder
struct ActiveRecordingInternal {
    encoder: Box<dyn AudioEncoder>,
    info: ActiveRecording,
}

/// Aufnahme-Engine verwaltet aktive Aufnahmen
pub struct RecordingEngine {
    /// Aktive Aufnahmen (Source-ID → Recording)
    active: HashMap<String, ActiveRecordingInternal>,
    /// Ausgabe-Verzeichnis (Standard: ~/Recordings/inoX-MIX)
    output_dir: PathBuf,
}

impl RecordingEngine {
    /// Neue Recording-Engine erstellen
    pub fn new() -> Self {
        let output_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Recordings")
            .join("inoX-MIX");

        // Verzeichnis erstellen falls nicht vorhanden
        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            log::warn!("Konnte Recording-Verzeichnis nicht erstellen: {}", e);
        }

        Self {
            active: HashMap::new(),
            output_dir,
        }
    }

    /// Ausgabe-Verzeichnis setzen
    pub fn set_output_dir(&mut self, path: PathBuf) -> Result<(), String> {
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("Konnte Verzeichnis nicht erstellen: {}", e))?;
        self.output_dir = path;
        Ok(())
    }

    /// Aufnahme starten
    pub fn start(&mut self, source_id: &str, format: RecordingFormat) -> Result<(), String> {
        // Prüfen ob bereits aktiv
        if self.active.contains_key(source_id) {
            return Err(format!("Aufnahme für '{}' bereits aktiv", source_id));
        }

        // Dateinamen generieren: [Datum]_[Source].[ext]
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("System-Zeit Fehler: {}", e))?
            .as_secs();

        let date_str = chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .map(|dt| dt.format("%Y%m%d_%H%M%S").to_string())
            .unwrap_or_else(|| format!("{}", timestamp));

        let extension = match format {
            RecordingFormat::Wav => "wav",
            RecordingFormat::Flac => "flac",
            RecordingFormat::Ogg => "ogg",
        };

        let filename = format!("{}_{}.{}", date_str, source_id, extension);
        let path = self.output_dir.join(&filename);

        // Encoder erstellen
        let encoder: Box<dyn AudioEncoder> = match format {
            RecordingFormat::Wav => {
                Box::new(WavEncoder::new(path.clone())?)
            }
            RecordingFormat::Flac => {
                return Err("FLAC-Format noch nicht implementiert".to_string());
            }
            RecordingFormat::Ogg => {
                return Err("OGG-Format noch nicht implementiert".to_string());
            }
        };

        // Aufnahme-Info erstellen
        let info = ActiveRecording {
            source_id: source_id.to_string(),
            format,
            path: path.to_string_lossy().to_string(),
            start_time: timestamp,
            samples_written: 0,
        };

        // In HashMap speichern
        self.active.insert(source_id.to_string(), ActiveRecordingInternal {
            encoder,
            info,
        });

        log::info!("Aufnahme gestartet: {} → {:?}", source_id, path);
        Ok(())
    }

    /// Aufnahme stoppen
    pub fn stop(&mut self, source_id: &str) -> Result<RecordingInfo, String> {
        // Aufnahme aus HashMap entfernen
        let mut recording = self.active.remove(source_id)
            .ok_or_else(|| format!("Keine aktive Aufnahme für '{}'", source_id))?;

        // Encoder finalisieren
        recording.encoder.finalize()?;

        // Datei-Info auslesen
        let path = PathBuf::from(&recording.info.path);
        let size_bytes = std::fs::metadata(&path)
            .map(|m| m.len())
            .unwrap_or(0);

        // Dauer berechnen (Samples / Sample-Rate / Channels)
        let sample_rate = 48000.0;
        let channels = 2.0;
        let duration_secs = recording.info.samples_written as f32 / (sample_rate * channels);

        log::info!("Aufnahme gestoppt: {} — {:.1}s, {} bytes", source_id, duration_secs, size_bytes);

        Ok(RecordingInfo {
            path: recording.info.path,
            duration_secs,
            size_bytes,
        })
    }

    /// Audio-Samples an aktive Aufnahme schreiben
    pub fn write_samples(&mut self, source_id: &str, samples: &[f32]) -> Result<(), String> {
        if let Some(recording) = self.active.get_mut(source_id) {
            recording.encoder.write_samples(samples)?;
            recording.info.samples_written += samples.len();
        }
        Ok(())
    }

    /// Alle aktiven Aufnahmen abrufen
    pub fn get_active_recordings(&self) -> Vec<ActiveRecording> {
        self.active.values()
            .map(|r| r.info.clone())
            .collect()
    }

    /// Prüfen ob Source aktuell aufnimmt
    pub fn is_recording(&self, source_id: &str) -> bool {
        self.active.contains_key(source_id)
    }
}

impl Default for RecordingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_recording_engine_new() {
        let engine = RecordingEngine::new();
        assert_eq!(engine.active.len(), 0);
    }

    #[test]
    fn test_start_stop_recording() {
        let mut engine = RecordingEngine::new();
        let source_id = "test_bus";

        // Start recording
        let result = engine.start(source_id, RecordingFormat::Wav);
        assert!(result.is_ok());
        assert!(engine.is_recording(source_id));
        assert_eq!(engine.get_active_recordings().len(), 1);

        // Write some dummy samples
        let samples = vec![0.5_f32; 48000]; // 0.5 Sekunden @ 48kHz Stereo
        engine.write_samples(source_id, &samples).unwrap();

        // Stop recording
        let info = engine.stop(source_id).unwrap();
        assert!(!engine.is_recording(source_id));
        assert_eq!(engine.get_active_recordings().len(), 0);

        // Check file exists
        let path = PathBuf::from(&info.path);
        assert!(path.exists());

        // Cleanup
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_multiple_recordings() {
        let mut engine = RecordingEngine::new();

        // Start two recordings
        engine.start("bus1", RecordingFormat::Wav).unwrap();
        engine.start("bus2", RecordingFormat::Wav).unwrap();

        assert_eq!(engine.get_active_recordings().len(), 2);
        assert!(engine.is_recording("bus1"));
        assert!(engine.is_recording("bus2"));

        // Stop one
        let info1 = engine.stop("bus1").unwrap();
        assert_eq!(engine.get_active_recordings().len(), 1);

        // Stop other
        let info2 = engine.stop("bus2").unwrap();
        assert_eq!(engine.get_active_recordings().len(), 0);

        // Cleanup
        fs::remove_file(&info1.path).ok();
        fs::remove_file(&info2.path).ok();
    }

    #[test]
    fn test_cannot_start_duplicate() {
        let mut engine = RecordingEngine::new();

        engine.start("bus1", RecordingFormat::Wav).unwrap();
        let result = engine.start("bus1", RecordingFormat::Wav);
        assert!(result.is_err());

        let info = engine.stop("bus1").unwrap();
        fs::remove_file(&info.path).ok();
    }

    #[test]
    fn test_flac_not_implemented() {
        let mut engine = RecordingEngine::new();
        let result = engine.start("test", RecordingFormat::Flac);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("nicht implementiert"));
    }
}
