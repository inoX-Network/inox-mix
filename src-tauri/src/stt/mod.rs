// Modul: stt — Sprache-zu-Text Manager (VOSK Live + Whisper Genau)
//
// Phase 1: VOSK für Live-STT (Task #6)
// Phase 2: Whisper für Offline-STT (Task #7)

pub mod vosk;
pub mod whisper;

use crate::config::database::Database;
use log::{info, warn};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use vosk::VoskEngine;
use whisper::WhisperEngine;

/// Erkanntes Wort mit Zeitstempel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognizedWord {
    /// Erkanntes Wort
    pub text: String,
    /// Start-Zeit in Sekunden
    pub start: f64,
    /// End-Zeit in Sekunden
    pub end: f64,
    /// Konfidenz (0.0 - 1.0)
    pub confidence: f32,
}

/// STT-Engine-Typ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SttEngineType {
    /// VOSK (Live, niedrige Latenz, ~100-300ms)
    Vosk,
    /// Whisper (Offline, hohe Genauigkeit, ~2-5s)
    Whisper,
}

impl SttEngineType {
    /// Engine-Name für UI
    pub fn name(&self) -> &'static str {
        match self {
            Self::Vosk => "VOSK",
            Self::Whisper => "Whisper",
        }
    }
}

/// Profanity Word (Schimpfwort-Eintrag)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfanityWord {
    /// ID in Datenbank
    pub id: i64,
    /// Wort (lowercase)
    pub word: String,
    /// Kategorie (schimpf, beleid, rass, custom)
    pub category: String,
    /// Sprache (de, en)
    pub language: String,
}

/// Profanity-Kategorie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfanityCategory {
    /// Schimpfwörter
    Schimpf,
    /// Beleidigungen
    Beleid,
    /// Rassistische Ausdrücke
    Rass,
    /// Custom (vom User hinzugefügt)
    Custom,
}

impl ProfanityCategory {
    /// Kategorie-String für Datenbank
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Schimpf => "schimpf",
            Self::Beleid => "beleid",
            Self::Rass => "rass",
            Self::Custom => "custom",
        }
    }

    /// Aus String parsen
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "schimpf" => Some(Self::Schimpf),
            "beleid" => Some(Self::Beleid),
            "rass" => Some(Self::Rass),
            "custom" => Some(Self::Custom),
            _ => None,
        }
    }
}

/// STT-Manager wählt zwischen VOSK (Live) und Whisper (Genau)
pub struct SttManager {
    /// VOSK-Instanz (Live-STT)
    vosk: Option<Arc<Mutex<VoskEngine>>>,
    /// Whisper-Instanz (Offline-STT)
    whisper: Option<Arc<Mutex<whisper::WhisperEngine>>>,
    /// Aktiver Engine-Typ
    active_engine: SttEngineType,
    /// Datenbank-Referenz für Profanity-Wörter
    db: Arc<Database>,
    /// Gecachte Profanity-Wörter (für schnelles Matching)
    profanity_words: Arc<Mutex<Vec<ProfanityWord>>>,
}

impl SttManager {
    /// Neuen STT-Manager erstellen
    ///
    /// # Parameter
    /// - `db`: Datenbank-Referenz für Profanity-Wörter
    /// - `vosk_model_path`: Pfad zum VOSK-Modell (optional)
    /// - `whisper_model_path`: Pfad zum Whisper-Modell (optional)
    pub fn new(
        db: Arc<Database>,
        vosk_model_path: Option<&str>,
        whisper_model_path: Option<&str>,
    ) -> Result<Self, String> {
        info!("STT-Manager wird initialisiert...");

        // Profanity-Wörter aus DB laden
        let profanity_words = Self::load_profanity_words(&db)?;
        info!("  {} Profanity-Wörter geladen", profanity_words.len());

        // VOSK-Engine initialisieren (wenn Modell-Pfad angegeben)
        let vosk = if let Some(path) = vosk_model_path {
            match VoskEngine::new(path, 16000.0) {
                Ok(engine) => {
                    info!("✓ VOSK-Engine geladen (Live-STT, ~100-300ms Latenz)");
                    Some(Arc::new(Mutex::new(engine)))
                }
                Err(e) => {
                    warn!("⚠ VOSK-Engine konnte nicht geladen werden: {}", e);
                    warn!("  VOSK Live-STT nicht verfügbar");
                    None
                }
            }
        } else {
            info!("  Kein VOSK-Modell-Pfad angegeben");
            None
        };

        // Whisper-Engine initialisieren (wenn Modell-Pfad angegeben)
        let whisper = if let Some(path) = whisper_model_path {
            match WhisperEngine::new(path, Some("de".to_string())) {
                Ok(engine) => {
                    info!("✓ Whisper-Engine geladen (Offline-STT, hohe Genauigkeit, ~2-5s Latenz)");
                    Some(Arc::new(Mutex::new(engine)))
                }
                Err(e) => {
                    warn!("⚠ Whisper-Engine konnte nicht geladen werden: {}", e);
                    warn!("  Whisper Offline-STT nicht verfügbar");
                    None
                }
            }
        } else {
            info!("  Kein Whisper-Modell-Pfad angegeben");
            None
        };

        // Standard-Engine wählen: VOSK bevorzugt (niedrige Latenz)
        let active_engine = if vosk.is_some() {
            SttEngineType::Vosk
        } else if whisper.is_some() {
            SttEngineType::Whisper
        } else {
            // Kein Engine verfügbar
            warn!("⚠ Keine STT-Engine verfügbar! Setze vosk_model_path oder whisper_model_path in Config");
            SttEngineType::Vosk // Fallback (wird aber nicht funktionieren)
        };

        info!("✓ STT-Manager initialisiert (Aktive Engine: {:?})", active_engine);

        Ok(Self {
            vosk,
            whisper,
            active_engine,
            db,
            profanity_words: Arc::new(Mutex::new(profanity_words)),
        })
    }

    /// Profanity-Wörter aus Datenbank laden
    fn load_profanity_words(db: &Database) -> Result<Vec<ProfanityWord>, String> {
        // SQL-Abfrage
        let query = "SELECT id, word, category, language FROM profanity_words ORDER BY word";

        db.query(query, [], |row| {
            Ok(ProfanityWord {
                id: row.get(0)?,
                word: row.get(1)?,
                category: row.get(2)?,
                language: row.get(3)?,
            })
        })
        .map_err(|e| format!("Fehler beim Laden von Profanity-Wörtern: {}", e))
    }

    /// Audio-Chunk für Echtzeit-STT verarbeiten
    ///
    /// # Rückgabe
    /// - `Some(Vec<String>)` wenn Wörter erkannt wurden
    /// - `None` wenn noch kein vollständiger Satz vorliegt
    ///
    /// # Hinweis
    /// - VOSK: Streaming-Mode, gibt Wörter zurück sobald erkannt
    /// - Whisper: Batch-Mode, sammelt Audio und transkribiert am Stück (höhere Latenz)
    pub fn process_audio(&mut self, samples: &[f32]) -> Option<Vec<String>> {
        match self.active_engine {
            SttEngineType::Vosk => {
                if let Some(vosk) = &self.vosk {
                    let mut engine = vosk.lock().unwrap();
                    engine.feed_audio(samples)
                } else {
                    None
                }
            }
            SttEngineType::Whisper => {
                if let Some(whisper) = &self.whisper {
                    let mut engine = whisper.lock().unwrap();
                    // Whisper verarbeitet das komplette Audio-Segment
                    match engine.transcribe(samples) {
                        Ok(words) => {
                            if !words.is_empty() {
                                Some(words)
                            } else {
                                None
                            }
                        }
                        Err(e) => {
                            warn!("Whisper Transkription fehlgeschlagen: {}", e);
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Erkannte Wörter gegen Profanity-Liste prüfen
    ///
    /// # Rückgabe
    /// - Liste der gefundenen Profanity-Wörter
    pub fn check_profanity(&self, words: &[String]) -> Vec<String> {
        let profanity_list = self.profanity_words.lock().unwrap();
        let mut matches = Vec::new();

        for word in words {
            let word_lower = word.to_lowercase();
            for profanity in profanity_list.iter() {
                if profanity.word == word_lower {
                    matches.push(word.clone());
                    break;
                }
            }
        }

        if !matches.is_empty() {
            info!("Profanity erkannt: {:?}", matches);
        }

        matches
    }

    /// Aktive Engine wechseln
    pub fn set_engine(&mut self, engine: SttEngineType) -> Result<(), String> {
        match engine {
            SttEngineType::Vosk => {
                if self.vosk.is_none() {
                    return Err("VOSK-Engine nicht verfügbar".to_string());
                }
            }
            SttEngineType::Whisper => {
                if self.whisper.is_none() {
                    return Err("Whisper-Engine nicht verfügbar (noch nicht implementiert)"
                        .to_string());
                }
            }
        }

        self.active_engine = engine;
        info!("STT-Engine gewechselt zu: {:?}", engine);
        Ok(())
    }

    /// Profanity-Wort hinzufügen
    pub fn add_profanity_word(
        &mut self,
        word: &str,
        category: ProfanityCategory,
        language: &str,
    ) -> Result<(), String> {
        let word_lower = word.to_lowercase();

        // In Datenbank einfügen
        self.db
            .execute(
                "INSERT INTO profanity_words (word, category, language) VALUES (?1, ?2, ?3)",
                params![word_lower, category.as_str(), language],
            )
            .map_err(|e| format!("Fehler beim Hinzufügen von Profanity-Word: {}", e))?;

        // Cache aktualisieren
        let profanity_words = Self::load_profanity_words(&self.db)?;
        *self.profanity_words.lock().unwrap() = profanity_words;

        info!("Profanity-Wort hinzugefügt: \"{}\" ({})", word_lower, category.as_str());
        Ok(())
    }

    /// Profanity-Wort entfernen
    pub fn remove_profanity_word(&mut self, word: &str) -> Result<(), String> {
        let word_lower = word.to_lowercase();

        // Aus Datenbank löschen
        self.db
            .execute("DELETE FROM profanity_words WHERE word = ?1", params![word_lower])
            .map_err(|e| format!("Fehler beim Entfernen von Profanity-Word: {}", e))?;

        // Cache aktualisieren
        let profanity_words = Self::load_profanity_words(&self.db)?;
        *self.profanity_words.lock().unwrap() = profanity_words;

        info!("Profanity-Wort entfernt: \"{}\"", word_lower);
        Ok(())
    }

    /// Alle Profanity-Wörter abrufen (optional gefiltert)
    pub fn get_profanity_words(
        &self,
        category: Option<ProfanityCategory>,
        language: Option<&str>,
    ) -> Vec<ProfanityWord> {
        let all_words = self.profanity_words.lock().unwrap();

        all_words
            .iter()
            .filter(|w| {
                if let Some(cat) = category {
                    if w.category != cat.as_str() {
                        return false;
                    }
                }
                if let Some(lang) = language {
                    if w.language != lang {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }

    /// Aktive Engine abrufen
    pub fn active_engine(&self) -> SttEngineType {
        self.active_engine
    }

    /// VOSK verfügbar?
    pub fn is_vosk_available(&self) -> bool {
        self.vosk.is_some()
    }

    /// Whisper verfügbar?
    pub fn is_whisper_available(&self) -> bool {
        self.whisper.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profanity_category() {
        assert_eq!(ProfanityCategory::Schimpf.as_str(), "schimpf");
        assert_eq!(ProfanityCategory::Beleid.as_str(), "beleid");
        assert_eq!(ProfanityCategory::Rass.as_str(), "rass");
        assert_eq!(ProfanityCategory::Custom.as_str(), "custom");

        assert_eq!(ProfanityCategory::from_str("schimpf"), Some(ProfanityCategory::Schimpf));
        assert_eq!(ProfanityCategory::from_str("invalid"), None);
    }

    #[test]
    fn test_stt_engine_type() {
        assert_eq!(SttEngineType::Vosk.name(), "VOSK");
        assert_eq!(SttEngineType::Whisper.name(), "Whisper");
    }
}
