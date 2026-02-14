# Modul 01: Core / Übersicht & Vision

## Zweck
Tauri-Projekt-Grundgerüst, PipeWire-Verbindung, Basis-Architektur.

## Anforderungen
- Tauri 2.x Projekt mit Rust-Backend und React-Frontend
- PipeWire-Verbindung beim Start herstellen (pipewire-rs)
- Fehler wenn PipeWire nicht verfügbar → User-Meldung
- SQLite-Datenbank für Config initialisieren
- Fenster: 1200×800 Standard, resizable, min 600×400
- Titelleiste: "inoX-MIX v0.3" + Logo
- Font: Oxanium über Google Fonts
- Farbschema: Cyan + Orange (siehe DESIGN-SYSTEM.md)

## Rust-Dateien
- src-tauri/src/main.rs: Tauri Entry, PipeWire Init, DB Init
- src-tauri/src/audio/mod.rs: Audio Engine Modul-Deklaration
- src-tauri/src/config/mod.rs: Config + Database Init

## React-Dateien
- src/App.tsx: Root mit Layout (Header, TabBar, Content, Sidebar)
- src/main.tsx: Entry
- src/styles/globals.css: Tailwind + Oxanium Import

## Tauri Commands
- get_system_info() → { pipewire_version, sample_rate, buffer_size }
- get_config(key) → String
- set_config(key, value) → ()

## Tests
- PipeWire Verbindung aufbauen + trennen
- Config lesen/schreiben SQLite
- Fenster öffnet sich mit korrektem Titel

## Agent-Reihenfolge
1. ARCHITEKT: Projektstruktur + Cargo.toml + package.json
2. BACKEND-ENGINEER: main.rs, PipeWire Init, Config/DB
3. FRONTEND-ENGINEER: App.tsx, Layout-Shell, Globals
4. QUALITÄTSPRÜFER: Audit
