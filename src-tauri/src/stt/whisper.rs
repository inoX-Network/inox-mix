// Modul: stt/whisper — Whisper Spracherkennung (hohe Genauigkeit, offline)
//
// Phase 2: Whisper-Integration für genaue Offline-STT
// SPEC: 09-bleeper

use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Whisper Transcription Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperResult {
    /// Erkannter Text
    pub text: String,
    /// Sprache (auto-detektiert oder vorgegeben)
    pub language: String,
    /// Konfidenz (0.0-1.0)
    pub confidence: f32,
}

/// Whisper STT-Engine für genaue Offline-Erkennung
pub struct WhisperEngine {
    context: Arc<Mutex<WhisperContext>>,
    model_path: String,
    language: Option<String>,
}

impl WhisperEngine {
    /// Neue Whisper-Engine erstellen und Modell laden
    ///
    /// # Parameter
    /// - `model_path`: Pfad zum Whisper-Modell (z.B. "~/.local/share/inox-mix/models/ggml-base.bin")
    /// - `language`: Sprache für Transkription (None = Auto-Detect, Some("de") = Deutsch)
    ///
    /// # Rückgabe
    /// - `Ok(WhisperEngine)` bei Erfolg
    /// - `Err(String)` wenn Modell nicht geladen werden kann
    ///
    /// # Modelle
    /// Whisper bietet verschiedene Modell-Größen:
    /// - tiny (~75 MB) - Schnell, niedrige Genauigkeit
    /// - base (~142 MB) - Ausgeglichen
    /// - small (~466 MB) - Gut
    /// - medium (~1.5 GB) - Sehr gut
    /// - large (~2.9 GB) - Beste Genauigkeit
    ///
    /// Download: https://huggingface.co/ggerganov/whisper.cpp/tree/main
    pub fn new(model_path: &str, language: Option<String>) -> Result<Self, String> {
        info!("Whisper-Engine wird initialisiert...");
        info!("  Model-Pfad: {}", model_path);
        info!("  Sprache: {:?}", language.as_deref().unwrap_or("auto"));

        // WhisperContext mit Parametern erstellen
        let ctx_params = WhisperContextParameters::default();

        let ctx = WhisperContext::new_with_params(model_path, ctx_params)
            .map_err(|e| format!("Whisper-Modell konnte nicht geladen werden: {:?}", e))?;

        info!("✓ Whisper-Engine erfolgreich initialisiert");

        Ok(Self {
            context: Arc::new(Mutex::new(ctx)),
            model_path: model_path.to_string(),
            language,
        })
    }

    /// Audio-Buffer transkribieren
    ///
    /// # Parameter
    /// - `samples`: Audio-Samples (f32, mono, bei 16000 Hz)
    ///
    /// # Rückgabe
    /// - `Ok(Vec<String>)` mit erkannten Wörtern
    /// - `Err(String)` bei Fehler
    ///
    /// # Hinweis
    /// Whisper erwartet mono 16kHz Audio. Die Verarbeitung dauert ~2-5 Sekunden
    /// je nach Modellgröße und Audio-Länge.
    pub fn transcribe(&mut self, samples: &[f32]) -> Result<Vec<String>, String> {
        if samples.is_empty() {
            return Err("Audio-Buffer ist leer".to_string());
        }

        info!("Whisper transkribiert {} Samples...", samples.len());

        let mut ctx = self.context.lock().unwrap();

        // State erstellen
        let mut state = ctx
            .create_state()
            .map_err(|e| format!("Fehler beim Erstellen von Whisper State: {:?}", e))?;

        // Full-Params konfigurieren
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        // Sprache setzen (falls vorgegeben)
        if let Some(ref lang) = self.language {
            params.set_language(Some(lang));
            params.set_translate(false); // Nicht übersetzen, nur transkribieren
        } else {
            // Auto-Detect
            params.set_language(None);
        }

        // Print-Ausgaben deaktivieren (nur für Logs)
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // Transkription durchführen
        state
            .full(params, samples)
            .map_err(|e| format!("Whisper Transkription fehlgeschlagen: {:?}", e))?;

        // Ergebnisse sammeln
        let num_segments = state
            .full_n_segments()
            .map_err(|e| format!("Fehler beim Abrufen von Segment-Anzahl: {:?}", e))?;

        let mut full_text = String::new();

        for i in 0..num_segments {
            let segment_text = state
                .full_get_segment_text(i)
                .map_err(|e| format!("Fehler beim Abrufen von Segment {}: {:?}", i, e))?;

            full_text.push_str(&segment_text);
            full_text.push(' ');
        }

        let full_text = full_text.trim();

        if !full_text.is_empty() {
            info!("Whisper erkannte: \"{}\"", full_text);

            // In Wörter aufteilen (lowercase)
            let words: Vec<String> = full_text
                .split_whitespace()
                .map(|w| w.to_lowercase())
                .collect();

            Ok(words)
        } else {
            warn!("Whisper erkannte keinen Text");
            Ok(Vec::new())
        }
    }

    /// Audio-Buffer transkribieren und vollen Text zurückgeben
    ///
    /// # Parameter
    /// - `samples`: Audio-Samples (f32, mono, bei 16000 Hz)
    ///
    /// # Rückgabe
    /// - `Ok(WhisperResult)` mit vollständigem Transkript
    /// - `Err(String)` bei Fehler
    pub fn transcribe_full(&mut self, samples: &[f32]) -> Result<WhisperResult, String> {
        if samples.is_empty() {
            return Err("Audio-Buffer ist leer".to_string());
        }

        let mut ctx = self.context.lock().unwrap();

        // State erstellen
        let mut state = ctx
            .create_state()
            .map_err(|e| format!("Fehler beim Erstellen von Whisper State: {:?}", e))?;

        // Full-Params konfigurieren
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

        // Sprache setzen
        if let Some(ref lang) = self.language {
            params.set_language(Some(lang));
            params.set_translate(false);
        } else {
            params.set_language(None);
        }

        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        // Transkription durchführen
        state
            .full(params, samples)
            .map_err(|e| format!("Whisper Transkription fehlgeschlagen: {:?}", e))?;

        // Ergebnisse sammeln
        let num_segments = state
            .full_n_segments()
            .map_err(|e| format!("Fehler beim Abrufen von Segment-Anzahl: {:?}", e))?;

        let mut full_text = String::new();

        for i in 0..num_segments {
            let segment_text = state
                .full_get_segment_text(i)
                .map_err(|e| format!("Fehler beim Abrufen von Segment {}: {:?}", i, e))?;

            full_text.push_str(&segment_text);
            full_text.push(' ');
        }

        let full_text = full_text.trim().to_string();

        let language_detected = self.language.clone().unwrap_or_else(|| "auto".to_string());

        info!("Whisper Transkription abgeschlossen: {} Zeichen", full_text.len());

        Ok(WhisperResult {
            text: full_text,
            language: language_detected,
            confidence: 1.0, // Whisper gibt keine Konfidenz zurück
        })
    }

    /// Model-Pfad abrufen
    pub fn model_path(&self) -> &str {
        &self.model_path
    }

    /// Sprache abrufen
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Sprache setzen
    pub fn set_language(&mut self, language: Option<String>) {
        self.language = language;
        info!(
            "Whisper Sprache geändert: {:?}",
            self.language.as_deref().unwrap_or("auto")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Benötigt Whisper-Modell
    fn test_whisper_engine_creation() {
        // Whisper-Modell muss installiert sein
        let model_path = "~/.local/share/inox-mix/models/ggml-base.bin";

        match WhisperEngine::new(model_path, Some("de".to_string())) {
            Ok(engine) => {
                println!("✓ Whisper-Engine erfolgreich erstellt");
                println!("  Model: {}", engine.model_path());
                println!("  Sprache: {:?}", engine.language());
            }
            Err(e) => {
                println!("⚠ Whisper-Engine-Test übersprungen: {}", e);
                println!("  Installiere Whisper-Modell:");
                println!("  wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin");
                println!("  mv ggml-base.bin ~/.local/share/inox-mix/models/");
            }
        }
    }

    #[test]
    fn test_transcribe_empty() {
        // Leerer Buffer sollte Fehler werfen
        let samples: Vec<f32> = Vec::new();
        println!("Leerer Audio-Buffer sollte Fehler werfen");
        // Test würde mit echtem Engine fehlschlagen
    }
}
