# Modul 13: Soundboard

## Position
Unterer Bereich der Streamer-Sidebar.

## Features
- Grid-Layout mit Sound-Buttons
- Vordefinierte Sounds: Airhorn 📣, Rimshot 🥁, Applaus 👏, Fail 💥
- Plus-Button: Eigene Sounds hinzufügen (WAV/MP3/OGG)
- Pro Button: Hotkey frei definierbar
- Bus-Routing: Standard B1 (Stream), pro Button änderbar
- Lautstärke: Global + pro Button
- Overlap-Modus: Sounds überlappen oder unterbrechen sich

## Button-Style
- 38×38px, border-radius 5px
- Background: rgba(255,255,255,0.02)
- Border: 1px solid rgba(255,255,255,0.05)
- Hover: leichter Glow
- Active (playing): Orange Glow + Border

## Rust-Backend
- src-tauri/src/streamer/soundboard.rs:
  - Sound laden (WAV/MP3/OGG via rodio oder symphonia crate)
  - Playback auf gewählten Bus
  - Concurrent Playback Management

## Tauri Commands
- play_sound(sound_id)
- stop_sound(sound_id)
- add_sound(path, name, hotkey?, bus_id?)
- remove_sound(sound_id)
- get_sounds() → Vec<SoundEntry>
- set_sound_volume(sound_id, value)
- set_soundboard_volume(value)
