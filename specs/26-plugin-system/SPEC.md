# Modul 26: Plugin-System (Zukunft)

## Status: Vorbereitet, nicht Phase 1

## Geplante Plugins
- LADSPA/LV2 Plugin-Host (eigene FX-Module laden)
- Custom Voice FX Presets (Community-erstellte Presets)
- Soundboard-Packs (Download/Import)
- Theme-Packs (Community-Themes)
- Integration-Plugins (Twitch API, Streamlabs, YouTube)
- Stream Deck / MIDI API Erweiterungen

## Plugin-Format
- JSON-Manifest mit Metadaten
- Plugin-Verzeichnis: ~/.config/inox-mix/plugins/
- Versionierung + Kompatibilitäts-Check

## Sicherheit
- Plugins laufen sandboxed (kein Zugriff auf System)
- Signatur-Check optional
- Whitelist für bekannte Plugins
