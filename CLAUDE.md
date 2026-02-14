# CLAUDE.md - inoX-MIX Projektanweisungen für Claude Code

## Projekt: inoX-MIX
## Version: 0.3
## Typ: Desktop Audio-Mixer für Linux Streamer
## Letzte Aktualisierung: 2026-02-14

---

## Grundregeln

1. LIES IMMER ZUERST die relevanten Dateien unter docs/ und specs/
2. WEICHE NIEMALS von der Spezifikation ab ohne explizite Erlaubnis
3. FRAGE bei Unklarheiten BEVOR du implementierst
4. JEDE Funktion braucht einen Test
5. KEINE hardcodierten Werte - alles aus config
6. KOMMENTIERE Code auf Deutsch (Projektsprache: Deutsch)
7. UI-Texte auf Deutsch (i18n-ready für spätere Mehrsprachigkeit)

## Technologie-Stack

| Komponente | Technologie |
|-----------|-------------|
| Runtime | Tauri 2.x (Rust + WebView) |
| Frontend | React 18, TypeScript, Vite |
| Styling | Tailwind CSS + Oxanium Font |
| Backend/Audio | Rust (PipeWire Bindings) |
| Audio-System | PipeWire (pw-cli, pw-link, pw-loopback) |
| AI/ML | DeepFilterNet (Denoise), RNNoise (Fallback), VOSK (STT Live), Whisper (STT Genau) |
| Datenbank | SQLite (lokale Config/Presets) |
| API | REST + WebSocket (Actix-Web oder Axum) |
| Packaging | GitHub Releases + Flatpak |
| CI/CD | GitHub Actions |

## Farbschema (STRIKT)

| Verwendung | Farbe | Hex |
|-----------|-------|-----|
| Primär/Akzent | Cyan | #00e5ff |
| Sekundär/Warm | Orange | #ff8c00 |
| Fehler/Mute/REC/Clip | Rot | #ff1744 |
| Status OK | Grün | #4caf50 |
| Peak-Warnung | Amber | #e6a117 |
| Hintergrund | Fast-Schwarz | #08090b |
| Panel | Dunkelgrau | #0d0f13 |
| Strip | Grau | #111318 |
| Text | Hellgrau | #cccccc |

KEINE weiteren Farben. Cyan = Hardware/A-Busse/Master. Orange = Virtual/B-Busse/Stream.

## Agenten-System

Wenn du eine Rolle zugewiesen bekommst (z.B. "Du bist der FRONTEND-ENGINEER"),
dann lies die entsprechende Datei unter .claude/agents/ und folge den dortigen Regeln.

## Reihenfolge bei neuen Features

1. Lies die SPEC.md des Features unter specs/
2. Prüfe Abhängigkeiten in docs/ARCHITECTURE.md
3. Erstelle Dateien gemäß Struktur
4. Implementiere mit Tests
5. Erstelle Prüf-Checkliste

## Verbotene Aktionen

- KEINE npm/cargo Pakete installieren ohne Rückfrage
- KEINE bestehenden Dateien überschreiben ohne Backup
- KEINE Architektur-Änderungen ohne ARCHITEKT-Rolle
- KEINE Sicherheitsmaßnahmen deaktivieren
- KEINE Farben außerhalb des definierten Schemas verwenden

## Nummerierungssystem

Alle UI-Elemente bekommen IDs nach Schema:
- Buttons: btn-[modul]-NNN
- Sliders: sld-[modul]-NNN
- Knobs: knb-[modul]-NNN
- Faders: fdr-[modul]-NNN
- Toggles: tgl-[modul]-NNN
- Panels: pnl-[modul]-NNN
- Modals: mdl-[modul]-NNN

Module: strip, master, bus, app, route, stream, fx, bleep, sound, set

## Audio-Konventionen

- Sample Rate: 48000 Hz (Standard, konfigurierbar)
- Buffer Size: 256 Samples (Standard, konfigurierbar)
- Bit Depth: 32-bit float intern
- Latenz-Ziel: < 5ms
- Signalfluss: Input → FX Chain → Bus Routing → Output
- VU-Meter: Peak + RMS, Refresh 60fps
- dB-Skala: -50 bis +10 dB, -∞ bei 0%
