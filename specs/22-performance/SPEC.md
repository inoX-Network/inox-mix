# Modul 22: Performance & Optimierung

## Ziele
- Binary < 10 MB
- RAM < 50 MB idle
- CPU < 5% bei aktiver Chain (ein Mic)
- Latenz < 5ms (Audio-Pipeline)
- Start < 2 Sekunden

## Features
| Feature | Beschreibung |
|---------|-------------|
| Perf-Monitor | CPU, RAM, aktive Chains in Settings anzeigen |
| Backup/Restore | Config als JSON exportieren/importieren |
| Undo/Redo | Strg+Z/Y für Einstellungen |
| Clip-Indikator | VU bleibt rot bis Reset |

## Optimierungs-Regeln
- Audio-Thread: Kein alloc, kein lock, kein IO
- UI-Update: 60fps für VU, 30fps für Rest
- PipeWire: Batch-Updates statt einzelne Calls
- Lazy Loading: FX-Module erst laden wenn aktiviert

## Tauri Commands
- get_performance() → { cpu_percent, ram_mb, latency_ms, active_chains }
- undo()
- redo()
