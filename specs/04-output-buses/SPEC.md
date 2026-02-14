# Modul 04: Output-Busse

## Zweck
4 Standard-Ausgangsbusse (erweiterbar auf 8).

## Standard-Busse
| ID | Name | Typ | Farbe | PipeWire |
|----|------|-----|-------|----------|
| A1 | SPEAKERS | Physisch | Cyan | System Default Sink |
| A2 | HEADSET | Physisch | Cyan | Separates Device |
| B1 | STREAM | Virtuell | Orange | pw-loopback Sink "inoX-MIX B1 Stream" |
| B2 | VOIP | Virtuell | Orange | pw-loopback Sink "inoX-MIX B2 VoIP" |

## Pro Bus (Horizontale Leiste)
- ID Label (A1/A2/B1/B2) in Bus-Farbe, Bold
- Sub-Label (SPEAKERS/HEADSET/STREAM/VOIP)
- Horizontaler Volume-Slider
- dB-Anzeige (7px, Bus-Farbe)
- MUTE Button
- REC Button (Rot-Akzent)
- Top-Accent: 2px Bus-Farbe, 40% Opacity

## Layout
- Position: Unterhalb der Mixer-Strips als horizontale Reihe
- Flex: flex-wrap, min-width 120px pro Bus
- Gap: 4px

## Rust-Backend
- Virtuelle Geräte: pw-loopback für B1, B2 beim Start erstellen
- Volume pro Bus: Mixer-Node Volume
- Mute: Node Mute Property
- Gerätezuordnung A1/A2 über Config (Setup-Wizard)

## Tauri Commands
- get_buses() → Vec<OutputBus>
- set_bus_volume(bus_id, value)
- set_bus_mute(bus_id, muted)

## Tests
- Virtuelle Geräte B1/B2 existieren in PipeWire nach Start
- Volume-Änderung wirkt auf Audio
- Mute: Stille auf Bus
