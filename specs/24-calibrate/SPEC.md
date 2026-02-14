# Modul 24: Quick Calibrate & Auto-Tuning

## Quick Calibrate
- Button: "🎙️ QUICK CALIBRATE" im FX-Panel Header
- Klick → Info-Bar: "Sprich 10 Sekunden normal ins Mikrofon"
- START Button → Analyse läuft

## Analyse (10 Sekunden Sprachprobe)
- Rauschpegel messen (Stille-Abschnitte)
- Durchschnittspegel messen (Sprech-Abschnitte)
- S-Laute Frequenz erkennen
- Frequenzspektrum analysieren

## Automatisch gesetzt
| Parameter | Methode |
|-----------|---------|
| Gate Threshold | 6 dB über gemessenem Rauschpegel |
| Compressor Threshold | Basierend auf Durchschnittspegel |
| De-Esser Frequenz | Erkannt aus S-Laut Spektrum |
| EQ-Preset | Empfehlung: Male/Female/Podcast/Gaming |

## Auto-Button pro FX-Modul
- Verfügbar bei: Gate, AI Denoise, De-Esser, Compressor
- Kalibriert nur das einzelne Modul live
- Nicht verfügbar bei: HPF, Limiter, Auto-Gain, EQ (Presets statt Auto)

## Ergebnis
- Vorschlag wird angezeigt (vorher/nachher Werte)
- User kann: Akzeptieren, Anpassen, Verwerfen

## Rust-Backend
- src-tauri/src/calibrate/mod.rs:
  - capture_sample(duration_secs) → AudioAnalysis
  - suggest_params(analysis) → CalibrationResult
  - apply_calibration(strip_id, result)

## Tauri Commands
- start_calibration(strip_id) → startet 10s Capture
- get_calibration_result(strip_id) → CalibrationResult
- apply_calibration(strip_id)
- discard_calibration(strip_id)
