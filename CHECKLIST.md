# CHECKLIST.md — inoX-MIX Projektstruktur (ARCHITEKT)

**Erstellt:** 2026-02-14
**Gesamtanzahl neue Dateien:** 79

---

## Konfigurationsdateien (11)

| Datei | Zweck | Status |
|-------|-------|--------|
| `package.json` | NPM Dependencies (React 18, Tauri 2, Zustand 5, Tailwind) | ✅ Erstellt |
| `tsconfig.json` | TypeScript Konfiguration (strict, Pfad-Aliases) | ✅ Erstellt |
| `tsconfig.node.json` | TypeScript Config für Vite | ✅ Erstellt |
| `vite.config.ts` | Vite Build-Konfiguration (React Plugin, Tauri Host) | ✅ Erstellt |
| `tailwind.config.ts` | Tailwind Custom-Farben (Cyan, Orange, etc.) + Oxanium Font | ✅ Erstellt |
| `postcss.config.js` | PostCSS für Tailwind + Autoprefixer | ✅ Erstellt |
| `index.html` | HTML Entry-Point | ✅ Erstellt |
| `.gitignore` | Git-Ignore (node_modules, target, dist) | ✅ Erstellt |
| `src-tauri/Cargo.toml` | Rust Dependencies (Tauri 2, PipeWire, SQLite, etc.) | ✅ Erstellt |
| `src-tauri/build.rs` | Tauri Build-Script | ✅ Erstellt |
| `src-tauri/tauri.conf.json` | Tauri Window (1200×800, min 600×400), Updater Config | ✅ Erstellt |

---

## Rust Backend (28 Dateien)

### Core (1)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/main.rs` | Tauri Entry, PipeWire Init, DB Init, Command Handler | ✅ Erstellt |

### Audio Engine (4)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/audio/mod.rs` | Audio-Engine Modul (AudioEngine Struct) | ✅ Erstellt |
| `src-tauri/src/audio/pipewire.rs` | PipeWire-Session, Geräte-Verwaltung, Linking | ✅ Erstellt |
| `src-tauri/src/audio/mixer.rs` | Volume-Kontrolle, Mute, Routing (MixerState) | ✅ Erstellt |
| `src-tauri/src/audio/metering.rs` | VU-Meter, Peak/RMS Berechnung (MeteringEngine) | ✅ Erstellt |

### FX Chain (9)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/fx/mod.rs` | FX-Chain Manager, AudioEffect Trait, FxType Enum | ✅ Erstellt |
| `src-tauri/src/fx/hpf.rs` | Hi-Pass Filter (Trittschall-Entfernung) | ✅ Erstellt |
| `src-tauri/src/fx/denoise.rs` | AI Denoise (DeepFilterNet Integration) | ✅ Erstellt |
| `src-tauri/src/fx/gate.rs` | Noise Gate (Stumm bei Stille) | ✅ Erstellt |
| `src-tauri/src/fx/deesser.rs` | De-Esser (Zischlaute reduzieren) | ✅ Erstellt |
| `src-tauri/src/fx/eq.rs` | 3-Band EQ (Tiefen, Mitten, Höhen) | ✅ Erstellt |
| `src-tauri/src/fx/compressor.rs` | Dynamik-Kompressor | ✅ Erstellt |
| `src-tauri/src/fx/limiter.rs` | Brickwall Limiter (Clipping-Schutz) | ✅ Erstellt |
| `src-tauri/src/fx/autogain.rs` | Auto-Gain (LUFS-Ziel) | ✅ Erstellt |

### Streamer Features (5)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/streamer/mod.rs` | Streamer-Engine Koordination | ✅ Erstellt |
| `src-tauri/src/streamer/ducking.rs` | Sidechain-Ducking (Musik leiser bei Sprache) | ✅ Erstellt |
| `src-tauri/src/streamer/bleeper.rs` | Profanity Bleeper (5 Modi: Beep/Mute/Noise/Reverse/Custom) | ✅ Erstellt |
| `src-tauri/src/streamer/voice_fx.rs` | Stimm-Effekte (Robot, Vader, Chipmunk, Radio, Echo) | ✅ Erstellt |
| `src-tauri/src/streamer/soundboard.rs` | Soundboard (Sound-Pads, Hotkeys, rodio Playback) | ✅ Erstellt |

### STT (3)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/stt/mod.rs` | STT Manager (VOSK Live + Whisper Genau) | ✅ Erstellt |
| `src-tauri/src/stt/vosk.rs` | VOSK Echtzeit-Spracherkennung | ✅ Erstellt |
| `src-tauri/src/stt/whisper.rs` | Whisper Offline-Spracherkennung | ✅ Erstellt |

### Config & Datenbank (4)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/config/mod.rs` | Config Manager (AppConfig, get/set_config) | ✅ Erstellt |
| `src-tauri/src/config/presets.rs` | Preset-Verwaltung (Laden/Speichern/Listen) | ✅ Erstellt |
| `src-tauri/src/config/database.rs` | SQLite Verbindung und CRUD | ✅ Erstellt |
| `src-tauri/src/config/migration.rs` | DB-Schema Migrationen | ✅ Erstellt |

### API (3)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/api/mod.rs` | REST + WebSocket Server | ✅ Erstellt |
| `src-tauri/src/api/routes.rs` | API Endpoints (get_system_info, set_volume, etc.) | ✅ Erstellt |
| `src-tauri/src/api/websocket.rs` | WebSocket für Echtzeit-Updates | ✅ Erstellt |

### Recording (2)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/recording/mod.rs` | Recording Engine (Start/Stop) | ✅ Erstellt |
| `src-tauri/src/recording/encoder.rs` | Audio-Encoder (WAV, FLAC, OGG) | ✅ Erstellt |

### Calibrate (1)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/calibrate/mod.rs` | Quick Calibrate (Mikrofon-Kalibrierung) | ✅ Erstellt |

### Updater (1)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src-tauri/src/updater/mod.rs` | Update Manager (GitHub Releases) | ✅ Erstellt |

---

## React Frontend (40 Dateien)

### Core (4)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/main.tsx` | React Entry mit StrictMode | ✅ Erstellt |
| `src/App.tsx` | Root: Header + TabBar + Content + Sidebar Slot | ✅ Erstellt |
| `src/vite-env.d.ts` | Vite TypeScript Referenz | ✅ Erstellt |
| `src/styles/globals.css` | Tailwind Directives, Oxanium Import, Body-Styles, CSS-Variablen | ✅ Erstellt |

### Layout (3)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/layout/Header.tsx` | Top Bar (Logo, CPU, Latenz, Presets) | ✅ Erstellt |
| `src/components/layout/TabBar.tsx` | Tab-Navigation (Mixer, FX, Routing, etc.) | ✅ Erstellt |
| `src/components/layout/DockPanel.tsx` | Dockbares Panel-System | ✅ Erstellt |

### Mixer (7)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/mixer/Strip.tsx` | Input Channel Strip (Hardware/Virtual) | ✅ Erstellt |
| `src/components/mixer/Fader.tsx` | Vertikaler Lautstärke-Regler | ✅ Erstellt |
| `src/components/mixer/VUMeter.tsx` | VU-Meter (13 Segmente, farbig) | ✅ Erstellt |
| `src/components/mixer/Knob.tsx` | Drehregler (Gain, Pan) | ✅ Erstellt |
| `src/components/mixer/Slider.tsx` | Horizontaler Regler (FX-Parameter) | ✅ Erstellt |
| `src/components/mixer/BusButton.tsx` | Bus-Routing Toggle (A1, A2, B1, B2) | ✅ Erstellt |
| `src/components/mixer/FXButton.tsx` | FX-Chain Ein/Aus Toggle | ✅ Erstellt |

### Master (1)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/master/MasterSection.tsx` | Master-Fader, VU, Limiter | ✅ Erstellt |

### Analyzer (3)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/analyzer/SignalMonitor.tsx` | Haupt-Analysator (4 animierte Wellen) | ✅ Erstellt |
| `src/components/analyzer/WaveDisplay.tsx` | Einzelne animierte SVG-Wellenform | ✅ Erstellt |
| `src/components/analyzer/StreamMonitor.tsx` | Kompakter Stream-Monitor (Sidebar) | ✅ Erstellt |

### FX (3)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/fx/FXPanel.tsx` | FX-Chain Detail-Panel | ✅ Erstellt |
| `src/components/fx/FXModule.tsx` | Einzelnes FX-Modul mit Parametern | ✅ Erstellt |
| `src/components/fx/Calibrate.tsx` | Quick Calibrate UI | ✅ Erstellt |

### Output (1)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/output/OutputBus.tsx` | Ausgangs-Bus Strip (A1, A2, B1, B2) | ✅ Erstellt |

### Apps (1)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/apps/AppMixer.tsx` | Per-App Lautstärke (PipeWire Streams) | ✅ Erstellt |

### Routing (1)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/routing/RoutingMatrix.tsx` | Routing-Matrix (Inputs → Busse) | ✅ Erstellt |

### Streamer Sidebar (6)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/streamer/StreamSidebar.tsx` | Slide-out Sidebar (270px rechts) | ✅ Erstellt |
| `src/components/streamer/StreamMaster.tsx` | Stream-Fader + Monitor | ✅ Erstellt |
| `src/components/streamer/DuckingPanel.tsx` | Sidechain-Ducking UI | ✅ Erstellt |
| `src/components/streamer/BleeperPanel.tsx` | Profanity Bleeper UI | ✅ Erstellt |
| `src/components/streamer/VoiceFX.tsx` | Voice FX Auswahl | ✅ Erstellt |
| `src/components/streamer/Soundboard.tsx` | Sound-Pad Grid | ✅ Erstellt |

### Settings & Help (2)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/components/settings/SettingsPage.tsx` | Einstellungen (Audio, Geräte, UI) | ✅ Erstellt |
| `src/components/help/FAQPage.tsx` | Hilfe und FAQ | ✅ Erstellt |

### Hooks (4)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/hooks/useAudioEngine.ts` | Tauri IPC für Audio-Commands | ✅ Erstellt |
| `src/hooks/useMeeting.ts` | VU/Peak/RMS Echtzeit-Daten | ✅ Erstellt |
| `src/hooks/usePresets.ts` | Preset-Management | ✅ Erstellt |
| `src/hooks/useWebSocket.ts` | WebSocket für externe Controller | ✅ Erstellt |

### Stores (3)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/stores/mixerStore.ts` | Zustand: Mixer-State (Kanäle, Busse) | ✅ Erstellt |
| `src/stores/fxStore.ts` | Zustand: FX-Chain State | ✅ Erstellt |
| `src/stores/streamStore.ts` | Zustand: Streamer-State (Ducking, Bleeper, etc.) | ✅ Erstellt |

### Types (3)

| Datei | Zweck | Status |
|-------|-------|--------|
| `src/types/audio.ts` | Audio-Typen (AudioDevice, MeterData, ChannelConfig) | ✅ Erstellt |
| `src/types/fx.ts` | FX-Typen (FxType, FxParam, CalibrationResult) | ✅ Erstellt |
| `src/types/api.ts` | API-Typen (CommandResponse, SystemInfo, WsMessage) | ✅ Erstellt |

---

## Zusammenfassung

| Bereich | Anzahl | Status |
|---------|--------|--------|
| Konfigurationsdateien | 11 | ✅ Alle erstellt |
| Rust Backend | 28 | ✅ Alle erstellt |
| React Frontend | 40 | ✅ Alle erstellt |
| **Gesamt** | **79** | **✅ Komplett** |

### Bestehendes (nicht verändert)

| Bereich | Dateien |
|---------|---------|
| docs/ | ARCHITECTURE.md, DESIGN-SYSTEM.md |
| specs/ | 26 Modul-Spezifikationen |
| .claude/ | agents/, skills/, templates/, PROMPTS.md |
| CLAUDE.md | Projektanweisungen |

---

## Nächster Schritt

```bash
# Initialer Commit
git add -A
git commit -m "scaffold: Tauri Projektstruktur + alle Skeletons"

# Rust Dependencies prüfen
cd src-tauri && cargo check
cd ..

# Frontend Dependencies installieren
npm install
npm run dev
```

Dann: **BACKEND-ENGINEER** für Modul 01-core (PipeWire Connection, SQLite Init, Basis-Config)
