# PROJECT-INVENTORY.md — inoX-MIX v0.3

> Automatisch generiert am 2026-02-15
> Jede Datei unter `src/` und `src-tauri/src/` dokumentiert

---

## INHALTSVERZEICHNIS

1. [Frontend: Stores (Zustand)](#1-stores-zustand)
2. [Frontend: Hooks](#2-hooks)
3. [Frontend: Types](#3-types)
4. [Frontend: Layout-Komponenten](#4-layout-komponenten)
5. [Frontend: Mixer-Komponenten](#5-mixer-komponenten)
6. [Frontend: Master-Sektion](#6-master-sektion)
7. [Frontend: Bus-Komponenten](#7-bus-komponenten)
8. [Frontend: FX-Komponenten](#8-fx-komponenten)
9. [Frontend: Streamer-Komponenten](#9-streamer-komponenten)
10. [Frontend: Analyzer-Komponenten](#10-analyzer-komponenten)
11. [Frontend: Routing](#11-routing)
12. [Frontend: Settings & Help](#12-settings--help)
13. [Frontend: Apps](#13-apps)
14. [Frontend: Output](#14-output)
15. [Backend: main.rs (Tauri Commands)](#15-backend-mainrs)
16. [Backend: audio/ Modul](#16-backend-audio-modul)
17. [Backend: fx/ Modul](#17-backend-fx-modul)
18. [Backend: config/ Modul](#18-backend-config-modul)
19. [Backend: streamer/ Modul](#19-backend-streamer-modul)
20. [Backend: stt/ Modul](#20-backend-stt-modul)
21. [Backend: recording/ Modul](#21-backend-recording-modul)
22. [Backend: calibrate/ Modul](#22-backend-calibrate-modul)
23. [Backend: updater/ Modul](#23-backend-updater-modul)
24. [Backend: api/ Modul](#24-backend-api-modul)
25. [Zusammenfassung](#25-zusammenfassung)

---

## 1. STORES (Zustand)

### `src/stores/appStore.ts` — 53 Zeilen — FERTIG

**State:**
```typescript
activeTab: AppTab           // 'mixer' | 'fx' | 'routing' | 'apps' | 'settings' | 'help'
sidebarOpen: boolean
systemInfo: SystemInfo | null
pipewireWarning: string | null
```

**Actions:**
- `setActiveTab(tab: AppTab)`
- `toggleSidebar()`
- `setSidebarOpen(open: boolean)`
- `setSystemInfo(info: SystemInfo)`
- `setPipewireWarning(warning: string | null)`

**invoke():** Keine | **listen():** Keine

---

### `src/stores/mixerStore.ts` — 151 Zeilen — FERTIG

**State:**
```typescript
strips: InputStrip[]
levels: Record<string, StripLevels>
loading: boolean
error: string | null
```

**Actions:**
- `loadStrips()` → invoke `'get_strips'`
- `setVolume(stripId, volumeDb)` → invoke `'set_strip_volume'`
- `setGain(stripId, gainDb)` → invoke `'set_strip_gain'`
- `setMute(stripId, muted)` → invoke `'set_strip_mute'`
- `setSolo(stripId, solo)` → invoke `'set_strip_solo'`
- `setBusRouting(stripId, busId, active)` → invoke `'set_strip_bus'`
- `addVirtualStrip()` → invoke `'add_virtual_strip'`
- `removeVirtualStrip(stripId)` → invoke `'remove_virtual_strip'`
- `updateLevels(levels)`

**invoke():** 8 Commands | **listen():** Keine

---

### `src/stores/fxStore.ts` — 75 Zeilen — FERTIG

**State:**
```typescript
modules: FxModuleInfo[]
loading: boolean
error: string | null
```

**Actions:**
- `loadFxChain()` → invoke `'get_fx_chain'`
- `setParam(moduleType, paramName, value)` → invoke `'set_fx_param'`
- `setBypass(moduleType, bypass)` → invoke `'set_fx_bypass'`

**invoke():** 3 Commands | **listen():** Keine

---

### `src/stores/busStore.ts` — 72 Zeilen — FERTIG

**State:**
```typescript
buses: OutputBus[]
loading: boolean
error: string | null
```

**Actions:**
- `loadBuses()` → invoke `'get_buses'`
- `setVolume(busId, volumeDb)` → invoke `'set_bus_volume'`
- `setMute(busId, muted)` → invoke `'set_bus_mute'`

**invoke():** 3 Commands | **listen():** Keine

---

### `src/stores/masterStore.ts` — 97 Zeilen — FERTIG

**State:**
```typescript
volume_db: number
limiter_ceiling_db: number
dim: boolean
mono: boolean
talkback: boolean
talkback_buses: string[]
loading: boolean
error: string | null
```

**Actions:**
- `loadMaster()` → invoke `'get_master'`
- `setVolume(volumeDb)` → invoke `'set_master_volume'`
- `setLimiter(ceilingDb)` → invoke `'set_master_limiter'`
- `setDim(active)` → invoke `'set_dim'`
- `setMono(active)` → invoke `'set_mono'`
- `setTalkback(active, targetBuses)` → invoke `'set_talkback'`

**invoke():** 6 Commands | **listen():** Keine

---

### `src/stores/routingStore.ts` — 78 Zeilen — FERTIG

**State:**
```typescript
entries: RoutingEntry[]
loading: boolean
error: string | null
```

**Actions:**
- `loadRoutingMatrix()` → invoke `'get_routing_matrix'`
- `setRouting(sourceId, busId, active)` → invoke `'set_routing'`
- `isRouted(sourceId, busId): boolean`

**invoke():** 2 Commands | **listen():** Keine

---

### `src/stores/streamStore.ts` — 47 Zeilen — SKELETON

Nur TypeScript Interfaces definiert, kein Zustand Store erstellt:
```typescript
interface StreamState { sidebarOpen, isLive, volume_db, muted }
interface DuckingState { enabled, amount_db, attack_ms, release_ms, threshold_db }
interface BleeperState { armed, mode, tone_hz, volume_db, engine }
interface VoiceFxState { enabled, preset, dry_wet }
```

**invoke():** Keine | **listen():** Keine

---

## 2. HOOKS

### `src/hooks/useMetering.ts` — 85 Zeilen — FERTIG

- **Exports:** `useMetering()`, `useStripMetering(stripId)`
- **listen():** `'metering-update'`
- Liefert `StripLevels[]` bzw. einzelne `StripLevels`

### `src/hooks/useAudioEngine.ts` — 16 Zeilen — SKELETON

- Leere Struktur mit TODO-Kommentaren für Tauri IPC

### `src/hooks/useMeeting.ts` — 15 Zeilen — SKELETON

- Falsch benannt (sollte useMetering2 o.ä. sein), leere Struktur

### `src/hooks/usePresets.ts` — 16 Zeilen — SKELETON

- TODO: list_presets, load_preset, save_preset

### `src/hooks/useWebSocket.ts` — 16 Zeilen — SKELETON

- TODO: WebSocket-Verbindung für externe Controller

---

## 3. TYPES

| Datei | Zeilen | Exports | Status |
|-------|--------|---------|--------|
| `src/types/mixer.ts` | 58 | `StripType`, `InputStrip`, `StripLevels` | FERTIG |
| `src/types/bus.ts` | 27 | `BusType`, `OutputBus` | FERTIG |
| `src/types/fx.ts` | 356 | `FxModuleType`, `FxModuleInfo`, `FxParamMeta`, `FxModuleMeta`, `FX_MODULE_META` | FERTIG |
| `src/types/master.ts` | 20 | `MasterState` | FERTIG |
| `src/types/routing.ts` | 14 | `RoutingEntry` | FERTIG |
| `src/types/audio.ts` | 53 | `AudioDevice`, `MeterData`, `ChannelConfig`, `AUDIO_CONSTANTS` | FERTIG |
| `src/types/api.ts` | 51 | `CommandResponse<T>`, `SystemInfo`, `WsMessage`, `UpdateInfo` | FERTIG |
| `src/vite-env.d.ts` | 1 | Vite Client Types | FERTIG |

---

## 4. LAYOUT-KOMPONENTEN

### `src/main.tsx` — 11 Zeilen — FERTIG
- React Entry Point mit StrictMode

### `src/App.tsx` — 161 Zeilen — FERTIG
- **Komponenten:** `App`, `TabContent`, `PipewireWarningBanner`
- **Stores:** `useAppStore`, `useMixerStore`
- **invoke():** `'get_system_info'`
- **listen():** `'pipewire-warning'`, `'level_update'`

### `src/components/layout/Header.tsx` — 92 Zeilen — FERTIG
- **Props:** Keine
- **Stores:** `useAppStore`
- Logo mit CSS-Gradient, PW-Status-Dot, Audio-Info inline

### `src/components/layout/TabBar.tsx` — 72 Zeilen — FERTIG
- **Props (TabButton):**
  ```typescript
  interface TabButtonProps {
    id: AppTab;
    label: string;
    active: boolean;
    onClick: () => void;
  }
  ```
- **Stores:** `useAppStore`

### `src/components/layout/SidebarToggleTab.tsx` — 56 Zeilen — FERTIG
- **Stores:** `useAppStore`
- 24x90px, Gradient-BG, writing-mode vertical-rl

### `src/components/layout/RecordingControl.tsx` — 93 Zeilen — FERTIG
- **invoke():** `'get_recording_status'`, `'stop_recording'`, `'start_recording'`
- Format-Auswahl (WAV/FLAC)

### `src/components/layout/ScenesControl.tsx` — 145 Zeilen — FERTIG
- **invoke():** `'get_scenes'`, `'load_scene'`, `'save_scene'`, `'get_strips'`, `'get_buses'`, `'get_fx_chain'`, `'get_routing_matrix'`, `'get_master'`, `'get_voice_fx_state'`
- Szenen-Management mit Save-Modal

### `src/components/layout/DockPanel.tsx` — 179 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface DockPanelProps {
    title: string;
    children: React.ReactNode;
    initialDocked?: boolean;
    initialPosition?: { x: number; y: number };
    initialSize?: { width: number; height: number };
  }
  ```
- Undockbares Panel mit Drag & Resize

---

## 5. MIXER-KOMPONENTEN

### `src/components/mixer/Mixer.tsx` — 139 Zeilen — FERTIG
- **Komponenten:** `Mixer`, `SectionLabel`
- **Stores:** `useMixerStore`
- 3-Spalten: Hardware | SignalMonitor+Master | Virtual

### `src/components/mixer/Strip.tsx` — 172 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface StripProps {
    strip: InputStrip;
  }
  ```
- **Stores:** `useMixerStore`
- min-width 56px, Gain-Knob, VU, Fader, Bus-Routing, M/S

### `src/components/mixer/Fader.tsx` — 132 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface FaderProps {
    value: number;
    onChange: (value: number) => void;
    color: 'cyan' | 'orange';
    disabled?: boolean;
    height?: number;  // default: 90
  }
  ```
- Track 2px, Thumb 14x9px, Drag + Wheel

### `src/components/mixer/Knob.tsx` — 114 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface KnobProps {
    value: number;
    onChange: (value: number) => void;
    label: string;
    color: 'cyan' | 'orange';
    min?: number;   // default: -80
    max?: number;   // default: 12
    size?: number;  // default: 20
  }
  ```
- SVG-Kreis mit Nadel, Label oberhalb

### `src/components/mixer/VUMeter.tsx` — 61 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface VUMeterProps {
    peak: number;
    rms: number;
    color: 'cyan' | 'orange';
    height?: number;  // default: 70
  }
  ```
- 13 Segmente, 3.5px breit, 1px gap

### `src/components/mixer/BusButton.tsx` — 36 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface BusButtonProps {
    busId: string;
    active: boolean;
    onClick: () => void;
  }
  ```
- 15x11px, 4.5px font

### `src/components/mixer/FXButton.tsx` — 34 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface FXButtonProps {
    active: boolean;
    onClick: () => void;
  }
  ```
- Zeigt `FX ●` oder `FX ○`

### `src/components/mixer/Slider.tsx` — 146 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface SliderProps {
    value: number;
    label: string;
    color?: string;
    onChange: (value: number) => void;
    unit?: string;
    disabled?: boolean;
  }
  ```
- Horizontaler Slider (generisch)

### `src/components/mixer/SignalMonitor.tsx` — 115 Zeilen — FERTIG
- 4 animierte SVG-Wellen (HW L/R, VIRT L/R)
- Grid 20x20, strokeWidth 0.5

---

## 6. MASTER-SEKTION

### `src/components/master/MasterSection.tsx` — 216 Zeilen — FERTIG
- **Komponenten:** `MasterSection`, `MasterFader`
- **Stores:** `useMasterStore`
- Label 7px/800, VOL+LIM 22px nebeneinander, Fader 120px, DIM/MONO/TALK Chips

---

## 7. BUS-KOMPONENTEN

### `src/components/bus/BusSection.tsx` — 60 Zeilen — FERTIG
- **Stores:** `useBusStore`
- Section-Label mit .sl Style, flex-wrap Bus-Grid

### `src/components/bus/BusStrip.tsx` — 108 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface BusStripProps {
    bus: OutputBus;
  }
  ```
- **Stores:** `useBusStore`
- Horizontal: ID | Slider | dB | MUTE | REC

### `src/components/bus/BusSlider.tsx` — 121 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface BusSliderProps {
    value: number;
    onChange: (value: number) => void;
    color: 'cyan' | 'orange';
    disabled?: boolean;
  }
  ```
- Horizontal, 100px breit, Thumb 6x8px

---

## 8. FX-KOMPONENTEN

### `src/components/fx/FxPanel.tsx` — 143 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface FxPanelProps {
    onClose: () => void;
  }
  ```
- **Stores:** `useFxStore`
- Grid 4x2, Chain-Flow Chips, Quick Calibrate Button

### `src/components/fx/FxModule.tsx` — 147 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface FxModuleProps {
    module: FxModuleInfo;
  }
  ```
- **Stores:** `useFxStore`
- ID 6px, Name 5px, Toggle 22x11px, Parameter-Slider

### `src/components/fx/FxSlider.tsx` — 91 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface FxSliderProps {
    value: number;
    min: number;
    max: number;
    onChange: (value: number) => void;
    color: 'cyan' | 'orange';
    disabled?: boolean;
  }
  ```
- Track 5px, Thumb 6x8px

### `src/components/fx/Calibrate.tsx` — 245 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface CalibrateProps {
    channelId: string;
    onComplete?: (result: CalibrationResult) => void;
    onCancel?: () => void;
  }
  ```
- **invoke():** `'run_calibration'`
- 4 States: idle → recording → analyzing → complete

---

## 9. STREAMER-KOMPONENTEN

### `src/components/streamer/StreamSidebar.tsx` — 186 Zeilen — FERTIG
- 270px breit, padding 10px, Stream Master + Monitor + AudioProtection + VoiceFX + Soundboard

### `src/components/streamer/StreamMaster.tsx` — 132 Zeilen — FERTIG
- **invoke():** `'set_stream_volume'`, `'set_stream_mute'`
- VU + Fader + dB + MUTE/REC

### `src/components/streamer/StreamMonitor.tsx` — 162 Zeilen — FERTIG
- 4 Wellen: Output Level, Duck Envelope, Bleeper, Voice FX

### `src/components/streamer/AudioProtection.tsx` — 300 Zeilen — FERTIG
- **Interne Komponenten:** `MiniSlider`, `Chip`, `ModeTile`
- Ducking (4 Slider) + Bleeper (Engine/Lang/Modus/Kategorien/Tone/Vol)

### `src/components/streamer/VoiceFXTiles.tsx` — 94 Zeilen — FERTIG
- 6 Presets (Robot, Vader, Chipmunk, Mega, Echo, Radio), Knob 18px, Tile min-width 36px

### `src/components/streamer/BleeperPanel.tsx` — 181 Zeilen — FERTIG
- **invoke():** `'set_bleeper_armed'`, `'set_bleeper_mode'`, `'set_bleeper_tone'`, `'set_bleeper_volume'`

### `src/components/streamer/DuckingPanel.tsx` — 175 Zeilen — FERTIG
- **invoke():** `'set_ducking_enabled'`, `'set_ducking_amount'`, `'set_ducking_attack'`, `'set_ducking_release'`, `'set_ducking_threshold'`

### `src/components/streamer/Soundboard.tsx` — 171 Zeilen — FERTIG
- **invoke():** `'get_sounds'`, `'play_sound'`, `'stop_sound'`, `'set_soundboard_volume'`

### `src/components/streamer/VoiceFX.tsx` — 135 Zeilen — FERTIG
- **invoke():** `'set_voice_fx_preset'`, `'set_voice_fx_drywet'`

---

## 10. ANALYZER-KOMPONENTEN

### `src/components/analyzer/WaveDisplay.tsx` — 149 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface WaveDisplayProps {
    color: string;
    label?: string;
    duration?: number;
    waveType?: number;
  }
  ```
- Animierte SVG-Wellenform, 4 Shapes

### `src/components/analyzer/SignalMonitor.tsx` — 98 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface SignalMonitorProps {
    levels?: number[];
    compact?: boolean;
  }
  ```
- 4 Wellen (2x Cyan HW, 2x Orange Virtual)

### `src/components/analyzer/StreamMonitor.tsx` — 75 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface StreamMonitorProps {
    levels?: number[];
    peak?: number;
    rms?: number;
  }
  ```
- 2 Orange-Wellen + Stereo VU

---

## 11. ROUTING

### `src/components/routing/RoutingMatrix.tsx` — 142 Zeilen — FERTIG
- **Stores:** `useRoutingStore`, `useMixerStore`, `useBusStore`
- Grid-Matrix: Inputs/Apps x Output-Busse

---

## 12. SETTINGS & HELP

### `src/components/settings/SettingsPage.tsx` — 575 Zeilen — FERTIG
- **invoke():** `'get_audio_devices'`, `'set_config'`, `'get_config'`, `'export_config'`, `'import_config'`
- 5 Sektionen: Audio, Hotkeys, Aufnahme, Darstellung, System + Performance

### `src/components/settings/UpdateSection.tsx` — 203 Zeilen — FERTIG
- **invoke():** `'get_config'`, `'set_config'`, `'check_for_updates'`, `'install_update'`
- **listen():** `'update-progress'`, `'update-installed'`

### `src/components/help/FAQPage.tsx` — 233 Zeilen — FERTIG
- 5 Kategorien, 8 FAQ-Einträge, Suchfunktion

---

## 13. APPS

### `src/components/apps/AppMixer.tsx` — 183 Zeilen — SKELETON

- UI komplett mit Mock-Daten (Firefox, Spotify, Discord, OBS)
- **invoke():** Keine (TODO: per-App Volume/Mute/Bus-Routing via PipeWire)
- Keine Backend-Anbindung

---

## 14. OUTPUT

### `src/components/output/OutputBus.tsx` — 188 Zeilen — FERTIG
- **Props:**
  ```typescript
  interface OutputBusProps {
    busId: string;
    name: string;
    type: 'A' | 'B';
    initialVolume?: number;
    initialMuted?: boolean;
    initialRecording?: boolean;
  }
  ```
- **invoke():** `'set_bus_volume'`, `'set_bus_mute'`
- Fader + VU + Geräte-Dropdown (Typ A)

---

## 15. BACKEND: main.rs

### `src-tauri/src/main.rs` — 1105 Zeilen — FERTIG

**AppState:**
```rust
struct AppState {
    config_manager: ConfigManager,
    mixer: Mutex<MixerState>,
    buses: Mutex<BusManager>,
    fx_chain: Mutex<FxChain>,
    routing: Mutex<RoutingManager>,
    master: Mutex<MasterManager>,
    recording: Mutex<RecordingEngine>,
    scenes: SceneManager,
    soundboard: Mutex<SoundboardManager>,
    voice_fx: Mutex<VoiceFxManager>,
    ducking: Mutex<DuckingEngine>,
    bleeper: Mutex<BleeperEngine>,
    stt: Mutex<SttManager>,
    calibrate: Mutex<CalibrateEngine>,
    metering: Mutex<MeteringService>,
}
```

**63 #[tauri::command] Funktionen:**

| Gruppe | Commands | Anzahl |
|--------|----------|--------|
| System | `get_system_info`, `get_audio_devices`, `get_config`, `set_config`, `export_config`, `import_config` | 6 |
| Mixer | `get_strips`, `set_strip_volume`, `set_strip_gain`, `set_strip_mute`, `set_strip_solo`, `set_strip_bus`, `add_virtual_strip`, `remove_virtual_strip` | 8 |
| Bus | `get_buses`, `set_bus_volume`, `set_bus_mute` | 3 |
| FX | `get_fx_chain`, `set_fx_param`, `set_fx_bypass` | 3 |
| Routing | `get_routing_matrix`, `set_routing` | 2 |
| Master | `get_master`, `set_master_volume`, `set_master_limiter`, `set_dim`, `set_mono`, `set_talkback` | 6 |
| Recording | `start_recording`, `stop_recording`, `get_recording_status` | 3 |
| Scenes | `save_scene`, `load_scene`, `delete_scene`, `get_scenes` | 4 |
| Soundboard | `play_sound`, `stop_sound`, `add_sound`, `remove_sound`, `get_sounds`, `set_sound_volume`, `set_soundboard_volume` | 7 |
| Voice FX | `get_voice_fx_state`, `set_voice_fx_preset`, `set_voice_fx_enabled`, `set_voice_fx_drywet` | 4 |
| Ducking | `get_ducking_state`, `set_ducking_enabled`, `set_ducking_amount`, `set_ducking_attack`, `set_ducking_release`, `set_ducking_threshold` | 6 |
| Bleeper | `get_bleeper_state`, `set_bleeper_armed`, `set_bleeper_mode`, `set_bleeper_tone`, `set_bleeper_volume` | 5 |
| STT | `set_stt_engine`, `add_profanity_word`, `remove_profanity_word`, `get_profanity_words`, `get_stt_status` | 5 |
| Calibrate | `run_calibration` | 1 |

**Updater Commands (aus updater/mod.rs):** `check_for_updates`, `install_update` = 2

**Gesamt: 65 Commands**

---

## 16. BACKEND: audio/ Modul

### `src-tauri/src/audio/mod.rs` — 79 Zeilen — FERTIG
```rust
pub struct AudioEngine { pw_session: Option<PipeWireSession> }
pub fn new() -> Result<Self, Box<dyn std::error::Error>>
pub fn is_connected(&self) -> bool
pub fn status(&self) -> PipeWireStatus
pub fn shutdown(&mut self)
```

### `src-tauri/src/audio/pipewire.rs` — 905 Zeilen — FERTIG
```rust
pub struct AudioDevice { pub id: u32, pub name: String, pub device_type: String, pub channels: u32 }
pub enum PipeWireStatus { Disconnected, Connecting, Connected, Error(String) }
pub struct PipeWireInfo { pub version: String, pub running: bool, pub sample_rate: u32, pub buffer_size: u32 }
pub struct PipeWireSession { status, running, thread_handle }

pub fn connect() -> Result<Self, Box<dyn std::error::Error>>
pub fn status(&self) -> PipeWireStatus
pub fn is_connected(&self) -> bool
pub fn disconnect(&mut self)
pub fn get_pipewire_info() -> PipeWireInfo
pub fn check_pipewire_available() -> Result<(), String>
pub fn create_audio_link(source_id: &str, bus_id: &str) -> Result<(), String>
pub fn remove_audio_link(source_id: &str, bus_id: &str) -> Result<(), String>
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String>
pub fn create_virtual_bus_nodes() -> Result<(), String>
pub fn destroy_virtual_bus_nodes() -> Result<(), String>
```
**Crates:** pipewire, log, serde

### `src-tauri/src/audio/mixer.rs` — 530 Zeilen — FERTIG
```rust
pub enum StripType { Hardware, Virtual }
pub struct InputStrip {
    pub id, label, strip_type, device_id, volume_db, gain_db,
    muted, solo, pan, fx_enabled, bus_routing, icon, order
}
pub struct MixerState { strips: HashMap<String, InputStrip>, next_virtual_id: u32 }

pub fn new_hardware/new_virtual(id, label, icon, order) -> Self
pub fn get_strips(&self) -> Vec<InputStrip>
pub fn set_volume/gain/mute/solo/bus_routing/pan/fx_enabled(...)
pub fn add_virtual_strip(&mut self) -> Result<InputStrip, String>
pub fn remove_virtual_strip(&mut self, strip_id) -> Result<(), String>
```
**Crates:** log, serde | **Tests:** vorhanden

### `src-tauri/src/audio/bus.rs` — 268 Zeilen — FERTIG
```rust
pub enum BusType { Physical, Virtual }
pub struct OutputBus { pub id, name, bus_type, device_id, volume_db, muted, recording }
pub struct BusManager { buses: HashMap<String, OutputBus> }

pub fn new_physical/new_virtual(id, name) -> Self
pub fn get_buses(&self) -> Vec<OutputBus>
pub fn set_volume/mute/recording(...)
```
**Crates:** log, serde | **Tests:** vorhanden

### `src-tauri/src/audio/routing.rs` — 231 Zeilen — FERTIG
```rust
pub struct RoutingEntry { pub source_id, bus_id, active }
pub struct RoutingManager { matrix: HashMap<(String, String), bool> }

pub fn get_routing_matrix(&self) -> Vec<RoutingEntry>
pub fn set_routing/is_routed/get_source_routing/clear(...)
```
**Crates:** serde, log | **Tests:** vorhanden

### `src-tauri/src/audio/master.rs` — 325 Zeilen — FERTIG
```rust
pub struct MasterState { pub volume_db, limiter_ceiling_db, dim, mono, talkback, talkback_buses }
pub struct MasterManager { state: MasterState }

pub fn set_volume/limiter/dim/mono/talkback(...)
pub fn get_effective_volume_db(&self) -> f32
```
**Crates:** serde, log | **Tests:** vorhanden

### `src-tauri/src/audio/metering.rs` — 445 Zeilen — FERTIG
```rust
pub struct StripLevels { pub strip_id, peak_l, peak_r, rms_l, rms_r, clipping }
pub struct MeteringEngine { meters: HashMap<String, MeterState> }

pub fn process_buffer(&mut self, strip_id, samples, channels)
pub fn get_levels(&self) -> Vec<StripLevels>
```
**Crates:** serde | Peak/RMS Berechnung mit Peak-Hold

### `src-tauri/src/audio/metering_service.rs` — 442 Zeilen — FERTIG
```rust
pub struct MeteringService { engine, audio_buffers, running, thread_handle, cpal_thread_handle, use_real_audio }

pub fn start(app_handle: AppHandle) -> Self
pub fn register_strip/unregister_strip/reset_clipping/stop(...)
```
**Crates:** log, tauri, rand | Echtzeit-Metering mit CPAL + PipeWire Node Discovery

### `src-tauri/src/audio/capture.rs` — 176 Zeilen — SKELETON
```rust
pub struct AudioSample { pub left: f32, pub right: f32 }
pub struct AudioCaptureManager { streams }

pub fn start_capture/stop_capture/read_samples/shutdown(...)
```
Placeholder — wartet auf Phase 2d CPAL Integration

### `src-tauri/src/audio/cpal_capture.rs` — 342 Zeilen — FERTIG
```rust
pub struct CpalCaptureManager { streams, host }

pub fn list_input_devices(&self) -> Result<Vec<String>, String>
pub fn start_capture(&mut self, device, stream_id) -> Result<Arc<Mutex<VecDeque<AudioSample>>>, String>
pub fn stop_capture/shutdown(...)
```
**Crates:** cpal, log | F32/I16 Support

---

## 17. BACKEND: fx/ Modul

### `src-tauri/src/fx/mod.rs` — 407 Zeilen — FERTIG
```rust
pub enum FxModuleType { Hpf, Denoise, Gate, DeEsser, Eq, Compressor, Limiter, AutoGain }
pub struct FxModuleInfo { pub module_type, enabled, params: Vec<(String, f32)> }
pub struct FxChain { modules: Vec<Box<dyn AudioProcessor>> }

pub trait AudioProcessor: Send {
    fn module_type(&self) -> FxModuleType;
    fn process(&mut self, samples: &mut [f32]);
    fn set_param(&mut self, name: &str, value: f32) -> Result<(), String>;
    fn get_params(&self) -> Vec<(String, f32)>;
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
}
```

| Datei | Zeilen | Status |
|-------|--------|--------|
| `fx/hpf.rs` | 278 | FERTIG — Butterworth Hi-Pass Filter |
| `fx/denoise.rs` | 150 | FERTIG — Spectral Gate Noise Reduction |
| `fx/gate.rs` | 334 | FERTIG — Noise Gate (Attack/Hold/Release) |
| `fx/deesser.rs` | 136 | FERTIG — Sibilance Reduction |
| `fx/eq.rs` | 290 | FERTIG — 3-Band Parametric EQ |
| `fx/compressor.rs` | 173 | FERTIG — Dynamics Compression |
| `fx/limiter.rs` | 211 | FERTIG — Brickwall Limiter mit Look-Ahead |
| `fx/autogain.rs` | 291 | FERTIG — RMS-basierte Auto-Gain Normalisierung |

**Alle 8 FX-Module mit Tests.**

---

## 18. BACKEND: config/ Modul

### `src-tauri/src/config/mod.rs` — 202 Zeilen — FERTIG
```rust
pub struct ConfigManager { db: Arc<Database> }
pub fn get/set/delete/export_all/import_all(...)
```

### `src-tauri/src/config/database.rs` — 343 Zeilen — FERTIG
```rust
pub struct Database { conn: Mutex<Connection> }
```
SQLite mit Tabellen: config, scenes, soundboard, profanity_words

### `src-tauri/src/config/presets.rs` — 278 Zeilen — FERTIG (SceneManager) / SKELETON (PresetManager)
```rust
pub struct Scene { pub id, name, state_json, created_at, updated_at }
pub struct SceneInfo { pub id, name, created_at }
pub struct SceneManager { db: Arc<Database> }
```

### `src-tauri/src/config/migration.rs` — 22 Zeilen — SKELETON
- TODO: Schema-Migration-System

---

## 19. BACKEND: streamer/ Modul

### `src-tauri/src/streamer/mod.rs` — 27 Zeilen — SKELETON
- `StreamerEngine` nur mit TODO-Kommentaren

### `src-tauri/src/streamer/bleeper.rs` — 305 Zeilen — FERTIG
```rust
pub enum BleepMode { Beep, Mute, Noise, Reverse, Custom }
pub struct BleeperEngine { pub mode, tone_hz, volume_db, armed, sample_rate, phase }
pub fn censor(&mut self, samples: &mut [f32], start: usize, end: usize)
```
**Crates:** serde, rand | **Tests:** 11

### `src-tauri/src/streamer/ducking.rs` — 276 Zeilen — FERTIG
```rust
pub struct DuckingParams { pub amount_db, attack_ms, release_ms, threshold_db }
pub struct DuckingEngine { pub params, enabled, sample_rate, envelope, attack_coeff, release_coeff }
pub fn process(&mut self, audio: &mut [f32], sidechain: &[f32])
```
**Crates:** serde | **Tests:** 8

### `src-tauri/src/streamer/soundboard.rs` — 298 Zeilen — FERTIG
```rust
pub struct SoundEntry { pub id, name, file_path, hotkey, bus_id, volume_db, created_at }
pub struct SoundboardManager { db, master_volume_db }
pub fn add_sound/play_sound/stop_sound/remove_sound/get_sounds(...)
```
**Crates:** rodio, rusqlite, serde, uuid | **Tests:** 8
**Bug:** `stop_sound()` nicht implementiert (benötigt Sink-Tracking)

### `src-tauri/src/streamer/voice_fx.rs` — 230 Zeilen — FERTIG
```rust
pub enum VoiceFxPreset { None, Robot, Vader, Chipmunk, Megaphone, Echo, Radio }
pub struct VoiceFxState { pub preset, enabled, dry_wet }
pub struct VoiceFxManager { state, engine }
pub fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<(), String>
```
**Tests:** 7

### `src-tauri/src/streamer/voice_fx_engine.rs` — 259 Zeilen — FERTIG
```rust
pub struct VoiceFxEngine { loader, active_chain, sample_rate, enabled }
pub fn set_preset(&mut self, preset: VoiceFxPreset) -> Result<(), String>
pub fn process(&mut self, input: &[f32], output: &mut [f32]) -> Result<(), String>
```
LADSPA-Plugin-Chain Integration

### `src-tauri/src/streamer/ladspa_loader.rs` — 261 Zeilen — FERTIG
```rust
pub struct LoadedPlugin { pub descriptor_ptr, library, library_path, unique_id, label, name, port_count }
pub struct LadspaLoader { plugins, libraries, scan_paths }
pub fn scan_plugins(&mut self) -> Result<usize, String>
```
**Crates:** libloading, log, shellexpand

### `src-tauri/src/streamer/ladspa_instance.rs` — 282 Zeilen — FERTIG
```rust
pub struct LadspaInstance { descriptor, handle, sample_rate, activated, ... }
pub fn process_mono/process_stereo(&mut self, input, output) -> Result<(), String>
```

### `src-tauri/src/streamer/ladspa_ffi.rs` — 112 Zeilen — FERTIG
- LADSPA C FFI Bindings (`LADSPA_Descriptor`, etc.)

---

## 20. BACKEND: stt/ Modul

### `src-tauri/src/stt/mod.rs` — 400 Zeilen — FERTIG
```rust
pub struct RecognizedWord { pub text, start, end, confidence }
pub enum SttEngineType { Vosk, Whisper }
pub struct ProfanityWord { pub id, word, category, language }
pub struct SttManager { vosk, whisper, active_engine, db, profanity_words }
pub fn process_audio(&mut self, samples: &[f32]) -> Option<Vec<String>>
pub fn check_profanity(&self, words: &[String]) -> Vec<String>
```
**Crates:** rusqlite, serde, log

### `src-tauri/src/stt/vosk.rs` — 262 Zeilen — FERTIG
```rust
pub struct VoskEngine { model, recognizer, sample_rate, model_path }
pub fn feed_audio(&mut self, samples: &[f32]) -> Option<Vec<String>>
```
**Crates:** vosk, serde, log | Live ~100-300ms Latenz

### `src-tauri/src/stt/whisper.rs` — 272 Zeilen — FERTIG
```rust
pub struct WhisperEngine { context, model_path, language }
pub fn transcribe(&mut self, samples: &[f32]) -> Result<Vec<String>, String>
```
**Crates:** whisper_rs, serde, log | Offline ~2-5s Latenz

---

## 21. BACKEND: recording/ Modul

### `src-tauri/src/recording/mod.rs` — 292 Zeilen — FERTIG
```rust
pub enum RecordingFormat { Wav, Flac, Ogg }
pub struct RecordingInfo { pub path, duration_secs, size_bytes }
pub struct ActiveRecording { pub source_id, format, path, start_time, samples_written }
pub struct RecordingEngine { active, output_dir }
pub fn start/stop/write_samples/get_active_recordings(...)
```
**Tests:** 6

### `src-tauri/src/recording/encoder.rs` — 180 Zeilen — FERTIG (WAV) / SKELETON (FLAC/OGG)
```rust
pub trait AudioEncoder: Send { fn write_samples/finalize(...) }
pub struct WavEncoder { writer, path }   // FERTIG
pub struct FlacEncoder { wav_encoder }   // Nutzt externe `flac` CLI
pub struct OggEncoder { _path }          // Nicht implementiert
```
**Crates:** hound

---

## 22. BACKEND: calibrate/ Modul

### `src-tauri/src/calibrate/mod.rs` — 50 Zeilen — SKELETON
```rust
pub struct CalibrationResult { pub recommended_gain_db, noise_floor_db, recommended_gate_db, recommended_hpf_hz }
pub struct CalibrateEngine { /* TODO */ }
pub fn run_calibration(&mut self, _samples: &[f32]) -> Result<CalibrationResult, String>
```
Nur Mock-Ergebnis, keine echte Analyse

---

## 23. BACKEND: updater/ Modul

### `src-tauri/src/updater/mod.rs` — 180 Zeilen — FERTIG
```rust
pub struct UpdateInfo { pub version, notes, url, date, available }
pub struct UpdateManager { auto_check_enabled }

#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> Result<Option<UpdateInfo>, String>
#[tauri::command]
pub async fn install_update(app: AppHandle, window: Window) -> Result<(), String>
```
**Crates:** tauri, tauri_plugin_updater | **Tests:** 3

---

## 24. BACKEND: api/ Modul

### `src-tauri/src/api/mod.rs` — 26 Zeilen — SKELETON
```rust
pub struct ApiServer { /* TODO: HTTP, WebSocket, Port */ }
pub fn start(_port: u16) -> Result<Self, Box<dyn std::error::Error>>  // TODO
pub fn stop(&self) -> Result<(), Box<dyn std::error::Error>>          // TODO
```

### `src-tauri/src/api/routes.rs` — 19 Zeilen — SKELETON
```rust
pub fn get_system_info() -> Result<String, String>     // TODO
pub fn get_mixer_state() -> Result<String, String>     // TODO
pub fn set_volume(_channel_id, _volume_db) -> Result   // TODO
```

### `src-tauri/src/api/websocket.rs` — 28 Zeilen — SKELETON
```rust
pub struct WsConnection { /* TODO */ }
pub struct WsServer { /* TODO */ }
pub fn broadcast_levels(&self, _levels_json: &str)     // TODO
```

---

## 25. ZUSAMMENFASSUNG

### Dateizahlen

| Bereich | Dateien | Zeilen |
|---------|---------|--------|
| Frontend (TSX/TS) | 62 | 6.789 |
| Backend (Rust) | 43 | 10.693 |
| **Gesamt** | **105** | **17.482** |

### Status-Verteilung

| Status | Frontend | Backend | Gesamt |
|--------|----------|---------|--------|
| FERTIG | 54 | 35 | **89** |
| SKELETON | 8 | 8 | **16** |
| BROKEN | 0 | 0 | **0** |

### Frontend SKELETON-Dateien
1. `src/hooks/useAudioEngine.ts` — Leere Struktur
2. `src/hooks/useMeeting.ts` — Leere Struktur (falsch benannt)
3. `src/hooks/usePresets.ts` — Leere Struktur
4. `src/hooks/useWebSocket.ts` — Leere Struktur
5. `src/stores/streamStore.ts` — Nur Interfaces
6. `src/components/apps/AppMixer.tsx` — UI fertig, keine Backend-Anbindung

### Backend SKELETON-Dateien
1. `src-tauri/src/audio/capture.rs` — Placeholder
2. `src-tauri/src/config/migration.rs` — TODO
3. `src-tauri/src/calibrate/mod.rs` — Mock-Ergebnis
4. `src-tauri/src/streamer/mod.rs` — Nur Konstruktor
5. `src-tauri/src/api/mod.rs` — TODO
6. `src-tauri/src/api/routes.rs` — TODO
7. `src-tauri/src/api/websocket.rs` — TODO
8. `src-tauri/src/recording/encoder.rs` — FLAC/OGG nicht fertig

### Fehlende Module (vs. 26 Specs)

| Spec | Status |
|------|--------|
| 01 Core | FERTIG |
| 02 Input Strips | FERTIG |
| 03 Signal Chain (FX) | FERTIG (alle 8 Module) |
| 04 Output Buses | FERTIG |
| 05 App Mixer | SKELETON (UI da, Backend fehlt) |
| 06 Routing Matrix | FERTIG |
| 07 Streamer Mode | FERTIG |
| 08 Voice FX | FERTIG (LADSPA) |
| 09 Profanity Bleeper | FERTIG (VOSK + Whisper) |
| 10 Presets & Scenes | FERTIG (Scenes) / SKELETON (Presets) |
| 11 Recording | FERTIG (WAV) / SKELETON (FLAC, OGG) |
| 12 Master Section | FERTIG |
| 13 Soundboard | FERTIG |
| 14 Settings | FERTIG |
| 15 Help & FAQ | FERTIG |
| 16 Responsive Layout | TEILWEISE (Basis da, Breakpoints fehlen) |
| 17 Controllers & API | SKELETON (API komplett TODO) |
| 18 Theme System | NICHT IMPLEMENTIERT |
| 19 System Integration | TEILWEISE (PipeWire ja, Tray-Icon fehlt) |
| 20 Health Check | NICHT IMPLEMENTIERT |
| 21 Accessibility | TEILWEISE (ARIA Labels, Keyboard TODO) |
| 22 Performance | NICHT IMPLEMENTIERT (als eigenständiges Modul) |
| 23 Update System | FERTIG |
| 24 Quick Calibrate | SKELETON (Mock-Ergebnis) |
| 25 Compatibility | NICHT SEPARAT (läuft auf Arch/Fedora/Ubuntu) |
| 26 Plugin System | NICHT IMPLEMENTIERT |

### Bekannte Bugs

1. **soundboard/stop_sound()** — Nicht implementiert (benötigt Sink-Tracking für rodio)
2. **recording/encoder** — FLAC nutzt externe CLI, OGG komplett fehlend
3. **calibrate** — Gibt nur Mock-Werte zurück, keine echte Audio-Analyse
4. **capture.rs** — Alter Placeholder, ersetzt durch cpal_capture.rs (kann entfernt werden)
5. **useMeeting.ts** — Falsch benannt, Duplikat-Skeleton (kann entfernt werden)
6. **streamStore.ts** — Kein Store erstellt, nur Interfaces (Streamer-Features nutzen lokalen State)

### Cargo-Dependencies

| Crate | Version | Verwendung |
|-------|---------|------------|
| tauri | 2 | Framework + tray-icon |
| tauri-plugin-updater | 2 | Auto-Updates |
| pipewire | 0.8 | Audio-System |
| cpal | 0.15 | Audio-Capture |
| rodio | 0.19 | Soundboard Playback |
| hound | 3.5 | WAV Encoder |
| vosk | 0.3 | Live STT |
| whisper-rs | 0.12 | Offline STT |
| libloading | 0.8 | LADSPA Plugin Loading |
| rusqlite | 0.31 | SQLite (bundled) |
| tokio | 1 | Async Runtime |
| serde / serde_json | 1 | Serialisierung |
| log / env_logger | 0.4 / 0.11 | Logging |
| chrono | 0.4 | Zeitstempel |
| uuid | 1.0 | IDs (v4) |
| rand | 0.8 | Zufallswerte |
| ringbuf | 0.4 | Ring Buffer |
| crossbeam-channel | 0.5 | Concurrency |
| dirs | 5.0 | System-Verzeichnisse |
| shellexpand | 3.0 | Path-Expansion |
