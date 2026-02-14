# Modul 09: Profanity Bleeper

## Zweck
Automatische Schimpfwort-Erkennung und Überblendung in Echtzeit.

## Funktionsweise
1. Mic-Audio parallel durch STT-Engine (VOSK oder Whisper)
2. Treffer in Wortliste → Bleeper-Modus auf B1 auslösen
3. Latenz: 100-300ms (VOSK empfohlen für Live)

## Bleeper-Modi (Kacheln, nicht Dropdown)
| Modus | Icon | Beschreibung |
|-------|------|-------------|
| Beep | 🔊 | Sinuston 400-2000 Hz, Standard 1000 Hz |
| Mute | 🔇 | Stummschalten (kurze Stille) |
| Noise | 🌫️ | Weißes Rauschen über das Wort |
| Reverse | 🔃 | Wort rückwärts abspielen |
| Custom | 🎵 | Eigene WAV-Datei (max 2 Sek) |

## Konfiguration
- STT-Engine: VOSK (Chips: VOSK/WHISPER)
- Sprachen: DE 🇩🇪 + EN 🇬🇧 (einzeln aktivierbar)
- Kategorien (Chips): Schimpf, Beleid., Rass., Custom
- Custom-Wörter: In Settings hinzufügen/entfernen
- Tone-Slider: Frequenz (nur bei Beep-Modus aktiv), Orange
- Volume-Slider: Lautstärke (alle Modi), Orange

## Bus-Routing
- B1 Stream: Standardmäßig aktiv
- Andere Busse: Optional zuschaltbar

## Rust-Backend
- src-tauri/src/streamer/bleeper.rs: Bleeper Engine
- src-tauri/src/stt/vosk.rs: VOSK Integration
- src-tauri/src/stt/whisper.rs: Whisper Integration (optional)
- Wortlisten: SQLite Tabelle "profanity_words" mit Kategorie + Sprache

## Tauri Commands
- set_bleeper_mode(mode: "beep"|"mute"|"noise"|"reverse"|"custom")
- set_bleeper_enabled(enabled)
- set_bleeper_tone(freq_hz)
- set_bleeper_volume(value)
- set_stt_engine(engine: "vosk"|"whisper")
- add_profanity_word(word, category, language)
- remove_profanity_word(word)
- get_profanity_words(category?, language?) → Vec<ProfanityWord>
