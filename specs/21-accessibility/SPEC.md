# Modul 21: Accessibility

## Anforderungen
| Feature | Beschreibung |
|---------|-------------|
| Tastatur | Alle Controls per Tab erreichbar, logische Reihenfolge |
| Screen-Reader | ARIA-Labels auf allen Elementen, Orca (GNOME) kompatibel |
| Hochkontrast | Zusätzlicher Modus für Sehbehinderte |
| Zoom | Skalierbare UI, Zoom-Stufe in Settings |

## ARIA-Labels (Beispiele)
- Fader: aria-label="USB MIC Volume", aria-valuenow="75", aria-valuemin="0", aria-valuemax="100"
- Mute: aria-label="USB MIC Mute", aria-pressed="false"
- VU: aria-label="USB MIC Level -12 dB"
- Knob: aria-label="USB MIC Gain", role="slider"

## Tastatur-Navigation
- Tab: Nächstes Control
- Shift+Tab: Vorheriges
- Arrow Up/Down: Fader/Knob Wert +/-
- Space: Toggle (Mute, FX, etc.)
- Enter: Bestätigen
- Escape: Panel schließen
