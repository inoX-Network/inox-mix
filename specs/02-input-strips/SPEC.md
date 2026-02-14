# Modul 02: Input-Strips

## Zweck
Hardware- und virtuelle Eingabe-Kanäle mit Fader, VU, Gain, Bus-Routing.

## Anforderungen

### Hardware-Strips (3 Standard)
- USB MIC: Farbe Cyan, Icon 🎙️
- HEADSET: Farbe Cyan, Icon 🎧
- LINE IN: Farbe Cyan, Icon 🔌
- Jeder zugeordnet zu einem PipeWire-Device

### Virtual-Strips (2 Standard, erweiterbar bis 7)
- VIRTUAL 1: Farbe Orange, Icon ◆
- VIRTUAL 2: Farbe Orange, Icon ◇
- Plus-Button zum Hinzufügen (max 10 Strips gesamt)

### Pro Strip enthält (von oben nach unten)
1. Dock-Handle (6 Dots, Drag & Drop vorbereitet)
2. Top-Accent (2px, Kanal-Farbe, 45% Opacity)
3. Icon (Emoji, 11px)
4. Label (6px, Bold, Kanal-Farbe, letter-spacing 1px)
5. Gain-Knob (20px, Kanal-Farbe, Label "GAIN")
6. Dual VU-Meter (links/rechts neben Fader, 13 Segmente, Farbe→Amber→Rot)
7. Fader (vertikal, 90px Höhe, Thumb 14×9px)
8. dB-Anzeige (7px, Kanal-Farbe, 1 Dezimal)
9. FX-Button (klickbar, öffnet FX-Panel, cyan wenn aktiv)
10. Bus-Routing (4 Mini-Buttons: A1, A2, B1, B2, farbig wenn aktiv)
11. Mute/Solo Buttons (M/S)

### Strip-Dimensionen
- Min-Width: 56px
- Background: #111318
- Border: 1px solid rgba(255,255,255,0.05)
- Border-Radius: 5px
- Padding: 8px 4px
- Gap: 4px zwischen Elementen

## Rust-Backend
- src-tauri/src/audio/mixer.rs:
  - struct InputStrip { id, label, device_id, volume, gain, muted, solo, bus_routing }
  - fn set_volume(strip_id, value) → PipeWire Node Volume ändern
  - fn set_gain(strip_id, value) → PipeWire Node Gain ändern
  - fn set_mute(strip_id, muted) → PipeWire Node Mute
  - fn set_bus_routing(strip_id, bus_id, active) → PipeWire Link erstellen/entfernen

## React-Frontend
- src/components/mixer/Strip.tsx: Kompletter Channel Strip
- src/components/mixer/Fader.tsx: Vertikaler Fader (Drag + Scroll)
- src/components/mixer/VUMeter.tsx: 13-Segment Meter (requestAnimationFrame)
- src/components/mixer/Knob.tsx: Rotary Knob (Drag)
- src/components/mixer/BusButton.tsx: Mini Bus-Routing Button

## Tauri Commands
- get_strips() → Vec<InputStrip>
- set_strip_volume(id, value)
- set_strip_gain(id, value)
- set_strip_mute(id, muted)
- set_strip_solo(id, solo)
- set_strip_bus(id, bus_id, active)
- add_virtual_strip() → InputStrip (max 10 check)
- remove_virtual_strip(id)

## Tauri Events
- level_update: { strip_id, peak_l, peak_r, rms_l, rms_r } @ 60fps

## Tests
- Strip erstellen, Volume setzen, Level ablesen
- Bus-Routing: Link erstellt/entfernt in PipeWire
- Mute: Signal auf 0
- Max 10 Strips Limit
- VU-Meter: Korrekte Peak/RMS Berechnung
