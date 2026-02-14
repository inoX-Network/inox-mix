# Modul 08: Voice FX / Stimmverzerrer

## Zweck
Echtzeit-Stimmverzerrung. NUR auf B1 (Stream) hörbar. Streamer hört sich normal.

## Presets (Kacheln im Grid)
| Preset | Effekt | Technik |
|--------|--------|---------|
| Robot | Pitch-Quantisierung + Ring-Modulator | LADSPA |
| Vader/Deep | Pitch runter + Formant-Shift + Hall | LV2 |
| Chipmunk | Pitch hoch + Formant-Shift | LADSPA |
| Megaphone | Bandpass + Verzerrung | LADSPA |
| Echo/Cave | Hall + Delay | LV2 |
| Radio | Bandpass + Kompression + Rauschen | LADSPA |

## Bedienung
- Master-Toggle: ON/OFF
- Kacheln: Klick = aktiv (Glow-Effekt), immer nur ein Preset aktiv
- Dry/Wet Knob: 0-100%
- Hotkey pro Preset frei definierbar
- Label: "NUR B1"

## Technik
- PipeWire Filter-Chain mit LADSPA/LV2 Plugins
- Nur in den B1-Signalpfad einschleifen
- A-Busse unverändert (Streamer hört sich normal)

## Rust-Backend
- src-tauri/src/streamer/voice_fx.rs:
  - Preset laden: PipeWire Filter-Chain konfigurieren
  - Dry/Wet: Mix-Regler zwischen Original und FX-Signal
  - Bypass: Filter-Chain entfernen

## Tauri Commands
- set_voice_fx_preset(preset_name) 
- set_voice_fx_enabled(enabled)
- set_voice_fx_drywet(value: 0.0-1.0)
