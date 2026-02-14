# ARCHITECTURE.md - inoX-MIX Systemarchitektur

## Übersicht

inoX-MIX ist ein professioneller Audio-Mixer für Linux-Streamer. Drei Schichten:

```
┌─────────────────────────────────────────────────┐
│  FRONTEND (React 18 + TypeScript + Tailwind)    │
│  - Mixer UI, Panels, Sidebar, Monitors          │
│  - WebSocket für Echtzeit-Metering              │
├─────────────────────────────────────────────────┤
│  TAURI BRIDGE (Rust ↔ JS IPC)                   │
│  - Commands: get_levels, set_volume, etc.        │
│  - Events: level_update, state_change            │
├─────────────────────────────────────────────────┤
│  BACKEND (Rust)                                  │
│  - Audio Engine (PipeWire Bindings)              │
│  - FX Chain (DSP in Rust)                        │
│  - AI Models (DeepFilterNet, VOSK)               │
│  - Config/State (SQLite)                         │
│  - REST/WS API (externe Controller)              │
├─────────────────────────────────────────────────┤
│  SYSTEM (PipeWire)                               │
│  - Virtuelle Geräte (pw-loopback)                │
│  - Routing (pw-link)                             │
│  - App-Audio Capture                             │
└─────────────────────────────────────────────────┘
```

## Verzeichnisstruktur

```
inox-mix/
├── CLAUDE.md                    # Claude Code Regeln
├── src-tauri/                   # Rust Backend
│   ├── src/
│   │   ├── main.rs              # Tauri Entry
│   │   ├── audio/
│   │   │   ├── mod.rs           # Audio Engine
│   │   │   ├── pipewire.rs      # PipeWire Bindings
│   │   │   ├── mixer.rs         # Volume, Routing
│   │   │   └── metering.rs      # VU, Peak, RMS
│   │   ├── fx/
│   │   │   ├── mod.rs           # FX Chain Manager
│   │   │   ├── hpf.rs           # Hi-Pass Filter
│   │   │   ├── denoise.rs       # AI Denoise (DeepFilterNet)
│   │   │   ├── gate.rs          # Noise Gate
│   │   │   ├── deesser.rs       # De-Esser
│   │   │   ├── eq.rs            # 3-Band EQ
│   │   │   ├── compressor.rs    # Compressor
│   │   │   ├── limiter.rs       # Limiter
│   │   │   └── autogain.rs      # Auto-Gain (LUFS)
│   │   ├── streamer/
│   │   │   ├── mod.rs           # Streamer Features
│   │   │   ├── ducking.rs       # Sidechain Ducking
│   │   │   ├── bleeper.rs       # Profanity Bleeper
│   │   │   ├── voice_fx.rs      # Voice FX Engine
│   │   │   └── soundboard.rs    # Soundboard Player
│   │   ├── stt/
│   │   │   ├── mod.rs           # STT Manager
│   │   │   ├── vosk.rs          # VOSK Integration
│   │   │   └── whisper.rs       # Whisper Integration
│   │   ├── config/
│   │   │   ├── mod.rs           # Config Manager
│   │   │   ├── presets.rs       # Preset/Szenen System
│   │   │   ├── database.rs      # SQLite
│   │   │   └── migration.rs     # DB Migrations
│   │   ├── api/
│   │   │   ├── mod.rs           # REST + WebSocket Server
│   │   │   ├── routes.rs        # API Endpoints
│   │   │   └── websocket.rs     # WS für Controller
│   │   ├── recording/
│   │   │   ├── mod.rs           # Recording Engine
│   │   │   └── encoder.rs       # FLAC/WAV/OGG
│   │   ├── calibrate/
│   │   │   └── mod.rs           # Quick Calibrate
│   │   └── updater/
│   │       └── mod.rs           # Update Manager
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                         # React Frontend
│   ├── App.tsx                  # Root Component
│   ├── main.tsx                 # Entry
│   ├── components/
│   │   ├── layout/
│   │   │   ├── Header.tsx       # Top Bar
│   │   │   ├── TabBar.tsx       # Navigation
│   │   │   └── DockPanel.tsx    # Dockable Panels
│   │   ├── mixer/
│   │   │   ├── Strip.tsx        # Input Channel Strip
│   │   │   ├── Fader.tsx        # Fader Component
│   │   │   ├── VUMeter.tsx      # VU Meter
│   │   │   ├── Knob.tsx         # Rotary Knob
│   │   │   ├── Slider.tsx       # Horizontal Slider
│   │   │   ├── BusButton.tsx    # Bus Routing Button
│   │   │   └── FXButton.tsx     # FX Chain Toggle
│   │   ├── master/
│   │   │   └── MasterSection.tsx
│   │   ├── analyzer/
│   │   │   ├── SignalMonitor.tsx # Main Analyzer
│   │   │   ├── WaveDisplay.tsx  # Animated Wave
│   │   │   └── StreamMonitor.tsx# Sidebar Analyzer
│   │   ├── fx/
│   │   │   ├── FXPanel.tsx      # FX Detail Panel
│   │   │   ├── FXModule.tsx     # Single FX Module
│   │   │   └── Calibrate.tsx    # Quick Calibrate UI
│   │   ├── output/
│   │   │   └── OutputBus.tsx    # Output Bus Strip
│   │   ├── apps/
│   │   │   └── AppMixer.tsx     # Application Mixer
│   │   ├── routing/
│   │   │   └── RoutingMatrix.tsx
│   │   ├── streamer/
│   │   │   ├── StreamSidebar.tsx# Slide-out Sidebar
│   │   │   ├── StreamMaster.tsx # Stream Fader + Monitor
│   │   │   ├── DuckingPanel.tsx
│   │   │   ├── BleeperPanel.tsx
│   │   │   ├── VoiceFX.tsx
│   │   │   └── Soundboard.tsx
│   │   ├── settings/
│   │   │   └── SettingsPage.tsx
│   │   └── help/
│   │       └── FAQPage.tsx
│   ├── hooks/
│   │   ├── useAudioEngine.ts    # Tauri Audio IPC
│   │   ├── useMeeting.ts       # VU/Peak Data
│   │   ├── usePresets.ts        # Preset Management
│   │   └── useWebSocket.ts      # External Controller WS
│   ├── stores/
│   │   ├── mixerStore.ts        # Zustand: Mixer State
│   │   ├── fxStore.ts           # Zustand: FX Chain State
│   │   └── streamStore.ts       # Zustand: Streamer State
│   ├── styles/
│   │   └── globals.css          # Tailwind + Custom
│   └── types/
│       ├── audio.ts             # Audio Type Definitions
│       ├── fx.ts                # FX Types
│       └── api.ts               # API Types
├── docs/                        # Spezifikationen
│   ├── ARCHITECTURE.md          # Diese Datei
│   ├── DESIGN-SYSTEM.md         # UI/Design Spec
│   └── API-SPEC.md              # REST/WS API Docs
├── specs/                       # Modul-Specs (pro Modul)
│   ├── 01-core/SPEC.md
│   ├── 02-input-strips/SPEC.md
│   └── ...
├── .claude/                     # Agent System
│   ├── agents/
│   ├── skills/
│   └── templates/
├── package.json
├── tsconfig.json
├── tailwind.config.ts
└── vite.config.ts
```

## Signal-Fluss

```
Physisch Input (Mic, Line) ──┐
                              ├─→ Input Strip ─→ FX Chain ─→ Bus Matrix ─→ Output Bus ─→ Physisch (Speaker, Headset)
App Audio (Firefox, OBS) ────┘                                    │                  └─→ Virtuell (Stream, VoIP)
                                                                  │
                                                    Streamer Sidebar:
                                                    - Ducking (Sidechain)
                                                    - Voice FX (nur B1)
                                                    - Bleeper (nur B1)
                                                    - Soundboard (→ B1)
```

## Implementierungs-Reihenfolge

### Phase 1: Fundament
1. Tauri-Projekt Setup
2. PipeWire Connection
3. Basis Input/Output Routing
4. VU Metering

### Phase 2: Mixer Core
5. Input Strips (Hardware + Virtual)
6. Fader/Volume Control
7. Bus Routing Matrix
8. Output Buses
9. App Mixer (PipeWire App Capture)
10. Master Section

### Phase 3: FX Chain
11. FX Chain Manager
12. Hi-Pass Filter
13. Noise Gate
14. EQ (3-Band)
15. Compressor
16. Limiter
17. Auto-Gain
18. AI Denoise (DeepFilterNet)
19. De-Esser
20. Quick Calibrate

### Phase 4: Streamer
21. Ducking/Sidechain
22. Voice FX Engine
23. Profanity Bleeper (VOSK STT)
24. Soundboard

### Phase 5: System
25. Presets & Szenen
26. Recording System
27. Settings
28. Theme System
29. Update System
30. External Controllers (API)
31. Health Check
