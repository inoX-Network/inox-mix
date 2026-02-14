# Modul 03: Signal-Chain (FX Chain)

## Zweck
8-stufige Audio-Verarbeitungskette pro Input-Strip.

## Chain-Reihenfolge (STRIKT)
1. Hi-Pass Filter (HPF) - Freq: 20-300 Hz, Standard 80 Hz
2. AI Denoise - Engine: DeepFilterNet/RNNoise, Strength: 0-100%
3. Noise Gate - Threshold: -60 bis 0 dB, Attack: 0.1-50ms, Hold: 0-500ms, Release: 5-500ms
4. De-Esser - Freq: 2-10 kHz, Reduction: 0-20 dB
5. Equalizer - 3-Band (Low/Mid/High), je ±12 dB
6. Compressor - Threshold: -50 bis 0 dB, Ratio: 1:1-20:1, Attack: 0.1-100ms, Release: 10-1000ms
7. Limiter - Ceiling: -12 bis 0 dB, Release: 1-500ms
8. Auto-Gain - Target: -24 bis -6 LUFS, Standard -14 (Streaming)

## Pro Modul
- On/Off Toggle (bypass)
- Parameter als horizontale Slider (im FX-Panel)
- Auto-Button bei: Gate, AI Denoise, De-Esser, Compressor
- FX-Farben: Abwechselnd Cyan und Orange (siehe DESIGN-SYSTEM.md)

## FX-Panel (Aufklappbar)
- Trigger: Klick auf "FX" Button im Strip
- Position: Unter den Output-Buses
- Header: Strip-Icon + Name + "QUICK CALIBRATE" Button + CLOSE
- Chain-Flow: Horizontale Kette HPF→AI-DN→GATE→DE-S→EQ→COMP→LIM→A-G
- Module: 4×2 Grid, je mit ID, Name, Toggle, Auto-Button, Parameter-Slider

## Rust-Backend
- src-tauri/src/fx/mod.rs: FxChain struct, AudioProcessor trait
- src-tauri/src/fx/hpf.rs: Butterworth 2nd-Order Hi-Pass
- src-tauri/src/fx/denoise.rs: DeepFilterNet FFI oder RNNoise
- src-tauri/src/fx/gate.rs: Noise Gate mit Envelope
- src-tauri/src/fx/deesser.rs: Sidechain-Filter + Compressor auf S-Frequenz
- src-tauri/src/fx/eq.rs: 3-Band parametric EQ (Biquad)
- src-tauri/src/fx/compressor.rs: Feed-forward Compressor
- src-tauri/src/fx/limiter.rs: Brick-Wall Limiter (Lookahead optional)
- src-tauri/src/fx/autogain.rs: LUFS-Messung + Gain-Anpassung

## Tauri Commands
- get_fx_chain(strip_id) → Vec<FxModule>
- set_fx_param(strip_id, module_id, param_name, value)
- set_fx_bypass(strip_id, module_id, bypass)
- auto_calibrate_module(strip_id, module_id) → CalibrateResult

## Tests
- Jedes FX-Modul: Bypass = Passthrough (Bit-identisch)
- HPF: Signal unter Cutoff gedämpft
- Gate: Signal unter Threshold stumm
- Compressor: Gain Reduction bei Signal über Threshold
- Limiter: Output nie über Ceiling
- Auto-Gain: Output-Level ≈ Target LUFS nach 10s
