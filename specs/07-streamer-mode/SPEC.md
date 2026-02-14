# Modul 07: Streamer-Modus (Sidebar)

## Zweck
Slide-Out Sidebar (rechts, 270px) mit allen Stream-spezifischen Features.

## Sidebar-Struktur (von oben nach unten)
1. Header: 📡 STREAMER + "Bus B1 — Stream Output" + LIVE Chip (Orange)
2. Stream Master + Monitor (nebeneinander)
   - Links: Stream Master Fader (VU + Fader + VU, dB, MUTE, REC)
   - Rechts: Stream Monitor (4 Wellen: Output Level Orange, Duck Envelope Cyan, Bleeper Rot, Voice FX Cyan)
3. Audio Protection Box (Ducking + Bleeper kombiniert)
   - Ducking: 4 Slider 2×2 (Amount, Attack, Release, Thresh), Trigger/Target Chips
   - Divider
   - Bleeper: Engine (VOSK/Whisper), Sprach-Flags, 5 Modus-Kacheln, Kategorien, Tone/Vol Slider
4. Voice FX: 6 Preset-Kacheln, Dry/Wet Knob
5. Soundboard: Sound-Buttons Grid

## Sidebar Toggle
- Tab-Button am rechten Rand: 📡 vertikal
- Klick = Sidebar slide-out (270px)
- Nochmal = slide-in

## Stream Monitor Wellen
- Output Level (Orange): Smooth Sine, 2.8s Animation
- Duck Envelope (Cyan): Rechteck-Muster (oben=normal, unten=geduckt), 3.5s
- Bleeper (Rot): Flache Linie mit Spikes bei Aktivität, 5s
- Voice FX (Cyan): High-Freq Muster, 2s

## OBS-Integration
- WebSocket-Bridge zu OBS
- OBS-Szenenwechsel → Mixer-Szene automatisch synchronisieren
- Config: OBS WS URL + Passwort in Settings

## React-Frontend
- src/components/streamer/StreamSidebar.tsx
- src/components/streamer/StreamMaster.tsx
- src/components/streamer/DuckingPanel.tsx
- src/components/streamer/BleeperPanel.tsx
- src/components/streamer/VoiceFX.tsx
- src/components/streamer/Soundboard.tsx
- src/components/analyzer/StreamMonitor.tsx
