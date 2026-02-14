# Modul 10: Presets & Szenen

## Presets (Quick-Configs)
| Preset | Beschreibung |
|--------|-------------|
| Streaming | Volle FX-Chain, Ducking, Bleeper, Voice FX |
| Podcast | Recording, Auto-Gain -16 LUFS |
| Gaming | Niedrige Latenz, Mic auf VoIP (B2) |
| Konferenz | Denoise + Gate, Sprache auf B2 |
| Custom | Eigene speichern/laden |

## Szenen (Kompletter Mixer-Zustand)
| Szene | Beschreibung |
|-------|-------------|
| Live-Stream | Mic laut, Game gedämpft, Bleeper aktiv |
| Just Chatting | Mic laut, Musik leise |
| BRB | Mic muted, Wartemusik auf Stream |
| Raid/Hype | Voice FX, Soundboard, Musik laut |
| Custom | Unbegrenzt eigene Szenen |

## Features
- Szenen-Wechsel per Hotkey mit optionalem Crossfade
- Szene = kompletter Snapshot: alle Volumes, Routing, FX, Mutes
- Import/Export als JSON
- Szenen sortierbar (Drag & Drop)

## Rust-Backend
- src-tauri/src/config/presets.rs:
  - Presets + Szenen in SQLite
  - save_preset(name, data), load_preset(name)
  - save_scene(name), load_scene(name), list_scenes()
  - Crossfade: Interpolation über N Frames

## Tauri Commands
- get_presets() → Vec<Preset>
- load_preset(name)
- save_scene(name)
- load_scene(name)
- delete_scene(name)
- get_scenes() → Vec<Scene>
