# Modul 19: System-Integration

## Virtuelle Geräte
- PipeWire Virtual Sources/Sinks beim Start erstellen
- Jedes PipeWire/PulseAudio-Programm sieht die Geräte
- Benennung: "inoX-MIX A1 Speakers", "inoX-MIX B1 Stream"
- Beim Beenden: Geräte aufräumen

## Tray-Icon
| Funktion | Beschreibung |
|----------|-------------|
| Quick-Mute | Ein-Klick Mic stummschalten |
| Szenen | Dropdown-Wechsel |
| Volume | Master direkt aus Tray |
| Status | Grün=OK, Rot=Error, Orange=Warning |

## Autostart
- Desktop-Entry in ~/.config/autostart/
- Optional in Settings

## Tauri Commands
- create_virtual_devices()
- cleanup_virtual_devices()
- set_tray_status(status: "ok"|"error"|"warning")
