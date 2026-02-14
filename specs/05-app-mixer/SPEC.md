# Modul 05: Application Mixer

## Zweck
Individuelle Lautstärke und Bus-Routing pro Anwendung.

## Auto-Erkennung
- PipeWire Registry: Neue Clients überwachen
- application.name Property für App-Identifikation
- Bekannte Apps mit Icon: Firefox 🌐, OBS 📹, Discord 💬, Spotify 🎵, Game Audio 🎮, System 🔔

## Pro App-Zeile
- App-Icon (Emoji, 13px)
- App-Name (8px, 600 weight, 70px breit)
- Volume-Slider (horizontal, flex:1, Cyan)
- Volume % (7px, Cyan)
- Bus-Routing Mini-Matrix (4 Buttons: A1, A2, B1, B2)
- Mute-Button

## Persistenz
- Einstellungen pro App in SQLite speichern
- Beim nächsten App-Start: Gespeicherte Config laden
- Optional: Auto-Profil bei Programmstart

## Tauri Commands
- get_apps() → Vec<AppEntry>
- set_app_volume(app_id, value)
- set_app_bus(app_id, bus_id, active)
- set_app_mute(app_id, muted)

## Tauri Events
- app_added: Neue App erkannt
- app_removed: App beendet
