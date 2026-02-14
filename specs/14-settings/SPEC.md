# Modul 14: Settings / Einstellungen

## Sektionen

### Audio (Cyan Accent)
- Geräte-Zuordnung (Input Strips ↔ PipeWire Devices)
- Samplerate: 44100 / 48000 / 96000 Hz
- Buffer: 64 / 128 / 256 / 512 / 1024 Samples
- Bit-Depth: 16 / 24 / 32-float

### Hotkeys (Orange Accent)
- Frei definierbar pro Aktion
- Aktionen: Mute Mic, Szenen-Wechsel, Voice FX Toggle, Push-to-Talk, Recording, DIM, Bleeper, Soundboard-Buttons

### Aufnahme (Orange Accent)
- Speicherort (Datei-Picker) + Speicherplatz-Anzeige
- Format: FLAC / WAV
- Dateinamen-Schema
- Auto-Aufnahme bei Stream-Start

### Profanity Bleeper
- Wortlisten: Hinzufügen/Entfernen/Import/Export
- Custom-Wörter verwalten
- Kategorien an/aus
- Bus-Zuordnung

### Darstellung (Cyan Accent)
- Sprache: DE / EN
- Theme: inoX Dark / System / inoX Light
- Layout: Standard / Erweitert / Kompakt
- VU-Meter Speed
- Fader-Empfindlichkeit

### System (Cyan Accent)
- Health-Check an/aus (Hintergrund/sichtbar)
- Auto-Start mit System
- Tray-Icon an/aus
- Auto-Update Prüfung an/aus
- Backup/Restore Config (JSON Export/Import)

### Performance (Orange Accent)
- CPU-Anzeige
- RAM-Anzeige
- Latenz-Anzeige (ms)

## Layout
- Sektionen als Panels (Accordion-Style)
- Toggle-Switches für An/Aus Werte
- Werte rechts-aligned

## Tauri Commands
- get_all_settings() → SettingsObject
- set_setting(section, key, value)
- export_config() → JSON String
- import_config(json: String)
- get_audio_devices() → Vec<AudioDevice>
