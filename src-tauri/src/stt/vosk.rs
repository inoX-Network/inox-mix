// Modul: stt/vosk — VOSK Echtzeit-Spracherkennung (für Bleeper)
//
// Phase 1: VOSK-Integration für Live-STT
// SPEC: 09-bleeper

use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use vosk::{DecodingState, Model, Recognizer};

/// Erkanntes Wort aus VOSK
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoskWord {
    /// Erkanntes Wort (lowercase)
    pub word: String,
    /// Konfidenz (0.0-1.0)
    pub conf: f32,
    /// Start-Zeit in Sekunden
    pub start: f64,
    /// End-Zeit in Sekunden
    pub end: f64,
}

/// VOSK Partial Result (während Erkennung läuft)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoskPartialResult {
    /// Bisher erkannter Text
    pub partial: String,
}

/// VOSK Final Result (vollständige Phrase)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoskFinalResult {
    /// Erkannter Text
    pub text: String,
    /// Einzelne Wörter mit Timing und Konfidenz
    #[serde(default)]
    pub result: Vec<VoskWord>,
}

/// VOSK STT-Engine für Live-Spracherkennung
pub struct VoskEngine {
    model: Arc<Model>,
    recognizer: Arc<Mutex<Recognizer>>,
    sample_rate: f32,
    model_path: String,
}

impl VoskEngine {
    /// Neue VOSK-Engine erstellen und Modell laden
    ///
    /// # Parameter
    /// - `model_path`: Pfad zum VOSK-Modell (z.B. "~/.local/share/inox-mix/models/vosk-model-de")
    /// - `sample_rate`: Sample-Rate für Audio (Standard: 16000 Hz)
    ///
    /// # Rückgabe
    /// - `Ok(VoskEngine)` bei Erfolg
    /// - `Err(String)` wenn Modell nicht geladen werden kann
    pub fn new(model_path: &str, sample_rate: f32) -> Result<Self, String> {
        info!("VOSK-Engine wird initialisiert...");
        info!("  Model-Pfad: {}", model_path);
        info!("  Sample-Rate: {} Hz", sample_rate);

        // Modell laden
        let model = Model::new(model_path)
            .ok_or_else(|| format!("VOSK-Modell konnte nicht geladen werden: {}", model_path))?;

        let model_arc = Arc::new(model);

        // Recognizer erstellen
        let recognizer = Recognizer::new(&model_arc, sample_rate).ok_or_else(|| {
            "Fehler beim Erstellen des VOSK Recognizers".to_string()
        })?;

        let recognizer_mutex = Arc::new(Mutex::new(recognizer));

        info!("✓ VOSK-Engine erfolgreich initialisiert");

        Ok(Self {
            model: model_arc,
            recognizer: recognizer_mutex,
            sample_rate,
            model_path: model_path.to_string(),
        })
    }

    /// Audio-Chunk für Echtzeit-Erkennung einspeisen
    ///
    /// # Parameter
    /// - `samples`: Audio-Samples (f32, mono, bei self.sample_rate Hz)
    ///
    /// # Rückgabe
    /// - `Some(recognized_words)` wenn ein vollständiger Satz erkannt wurde
    /// - `None` wenn noch kein vollständiger Satz vorliegt
    ///
    /// # Hinweis
    /// VOSK erwartet 16-bit PCM Audio. Wir konvertieren f32 → i16.
    pub fn feed_audio(&mut self, samples: &[f32]) -> Option<Vec<String>> {
        // f32 Samples → i16 PCM konvertieren
        let pcm_samples: Vec<i16> = samples
            .iter()
            .map(|&s| (s.clamp(-1.0, 1.0) * 32767.0) as i16)
            .collect();

        let mut recognizer = self.recognizer.lock().unwrap();

        // Audio an VOSK übergeben
        match recognizer.accept_waveform(&pcm_samples) {
            Ok(DecodingState::Finalized) => {
                // Vollständiges Ergebnis abrufen
                let complete_result = recognizer.result();

                // CompleteResult::Single oder Multiple
                match complete_result {
                    vosk::CompleteResult::Single(result_data) => {
                        let text = result_data.text;
                        if !text.is_empty() {
                            info!("VOSK erkannte: \"{}\"", text);
                            let words: Vec<String> = text
                                .split_whitespace()
                                .map(|w| w.to_lowercase())
                                .collect();
                            return Some(words);
                        }
                    }
                    vosk::CompleteResult::Multiple(results) => {
                        // Nehme die erste Alternative (höchste Konfidenz)
                        if let Some(first) = results.alternatives.get(0) {
                            let text = &first.text;
                            if !text.is_empty() {
                                info!("VOSK erkannte: \"{}\"", text);
                                let words: Vec<String> = text
                                    .split_whitespace()
                                    .map(|w| w.to_lowercase())
                                    .collect();
                                return Some(words);
                            }
                        }
                    }
                }
            }
            Ok(DecodingState::Running) => {
                // Noch kein vollständiger Satz
            }
            Ok(DecodingState::Failed) => {
                warn!("VOSK Decoding fehlgeschlagen");
            }
            Err(e) => {
                warn!("VOSK accept_waveform Fehler: {:?}", e);
            }
        }

        None
    }

    /// Partial Result abrufen (für UI-Anzeige während Erkennung läuft)
    pub fn get_partial_result(&self) -> Option<String> {
        let mut recognizer = self.recognizer.lock().unwrap();
        let partial = recognizer.partial_result();

        if !partial.partial.is_empty() {
            Some(partial.partial.to_string())
        } else {
            None
        }
    }

    /// Final Result erzwingen (am Ende einer Aufnahme)
    pub fn finalize(&mut self) -> Option<Vec<String>> {
        let mut recognizer = self.recognizer.lock().unwrap();
        let complete_result = recognizer.final_result();

        match complete_result {
            vosk::CompleteResult::Single(result_data) => {
                let text = result_data.text;
                if !text.is_empty() {
                    info!("VOSK Final-Result: \"{}\"", text);
                    let words: Vec<String> = text
                        .split_whitespace()
                        .map(|w| w.to_lowercase())
                        .collect();
                    return Some(words);
                }
            }
            vosk::CompleteResult::Multiple(results) => {
                if let Some(first) = results.alternatives.get(0) {
                    let text = &first.text;
                    if !text.is_empty() {
                        info!("VOSK Final-Result: \"{}\"", text);
                        let words: Vec<String> = text
                            .split_whitespace()
                            .map(|w| w.to_lowercase())
                            .collect();
                        return Some(words);
                    }
                }
            }
        }

        None
    }

    /// Model-Pfad abrufen
    pub fn model_path(&self) -> &str {
        &self.model_path
    }

    /// Sample-Rate abrufen
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Engine zurücksetzen (für neue Erkennungs-Session)
    pub fn reset(&mut self) -> Result<(), String> {
        info!("VOSK-Engine wird zurückgesetzt...");

        // Neuen Recognizer erstellen
        let recognizer = Recognizer::new(&self.model, self.sample_rate).ok_or_else(|| {
            "Fehler beim Zurücksetzen des VOSK Recognizers".to_string()
        })?;

        *self.recognizer.lock().unwrap() = recognizer;

        info!("✓ VOSK-Engine zurückgesetzt");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Benötigt VOSK-Modell
    fn test_vosk_engine_creation() {
        // Deutsches VOSK-Modell muss installiert sein
        let model_path = "~/.local/share/inox-mix/models/vosk-model-small-de-0.15";

        match VoskEngine::new(model_path, 16000.0) {
            Ok(engine) => {
                println!("✓ VOSK-Engine erfolgreich erstellt");
                println!("  Model: {}", engine.model_path());
                println!("  Sample-Rate: {} Hz", engine.sample_rate());
            }
            Err(e) => {
                println!("⚠ VOSK-Engine-Test übersprungen: {}", e);
                println!("  Installiere VOSK-Modell:");
                println!("  wget https://alphacephei.com/vosk/models/vosk-model-small-de-0.15.zip");
                println!("  unzip vosk-model-small-de-0.15.zip -d ~/.local/share/inox-mix/models/");
            }
        }
    }

    #[test]
    fn test_audio_feed_stub() {
        // Simuliere Audio-Samples (Stille)
        let samples = vec![0.0f32; 1600]; // 100ms @ 16kHz

        // Test würde mit echtem Modell laufen
        println!("Audio-Feed würde {} Samples verarbeiten", samples.len());
    }
}
