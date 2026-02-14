# Modul 11: Aufnahme-System

## Features
- Record per Output-Bus (REC-Button pro Bus)
- Record per Input-Strip (Einzelne Inputs für Post-Production)
- Multitrack: Mehrere gleichzeitig
- Formate: FLAC (Standard, ~50% kleiner) oder WAV (unkomprimiert)
- Dateinamen: Auto [Datum]_[Bus/Strip]_[Szene].flac
- Speicherort: Konfigurierbar in Settings (Standard ~/Recordings/inoX-MIX)
- Live-Indikator: Dauer + Dateigröße im UI
- Auto-REC: Optional bei Stream-Start (B1)

## Rust-Backend
- src-tauri/src/recording/mod.rs: Recording Manager
- src-tauri/src/recording/encoder.rs: FLAC/WAV Encoder
- Audio-Thread: Ring-Buffer → Encoder-Thread → Datei
- Kein Blocking auf Audio-Thread!

## Tauri Commands
- start_recording(source_id, format: "flac"|"wav")
- stop_recording(source_id) → RecordingInfo { path, duration, size }
- get_recording_status() → Vec<ActiveRecording>
