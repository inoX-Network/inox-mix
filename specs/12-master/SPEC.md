# Modul 12: Master-Sektion

## Position
Rechts neben dem Signal-Monitor im Mixer-Tab.

## Elemente (von oben nach unten)
1. Label: "MASTER" (10px, Cyan, letter-spacing 3px)
2. VOL Knob (24px, Cyan, Label "VOL")
3. LIM Knob (24px, Orange, Label "LIM")
4. Dual VU-Meter (links/rechts, Cyan)
5. Master Fader (vertikal, 120px)
6. dB-Anzeige (11px, Cyan, Bold)
7. Chips: DIM (Orange), MONO, TALK

## Funktionen
| Feature | Beschreibung |
|---------|-------------|
| Master Volume | Gesamtlautstärke aller Busse |
| Master Limiter | Letzte Sicherung gegen Clipping |
| DIM | Sofort -20 dB bei Unterbrechungen |
| Mono-Check | Mono-Summe für Podcast-Kompatibilität |
| Talkback | Mic auf ausgewählte Busse für Co-Host |
| Clip-Indikator | VU bleibt rot bis manuell Reset |

## Hintergrund
- Cyan Gradient + Glow Border
- Background: linear-gradient(135deg, rgba(0,229,255,0.04), rgba(0,229,255,0.01))
- Border: 1px solid rgba(0,229,255,0.08)

## Tauri Commands
- set_master_volume(value)
- set_master_limiter(ceiling_db)
- set_dim(active)
- set_mono(active)
- set_talkback(active, target_buses: Vec<String>)
