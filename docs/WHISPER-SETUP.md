# Whisper Speech-to-Text Setup

## Übersicht

inoX-MIX unterstützt Whisper für hochgenaue Offline-Spracherkennung (Profanity Bleeper). Whisper bietet höchste Genauigkeit, hat aber höhere Latenz (~2-5s) im Vergleich zu VOSK (~100-300ms).

## Wann Whisper verwenden?

| Use Case | VOSK | Whisper |
|----------|------|---------|
| Live-Streaming | ✅ Optimal | ⚠️ Zu langsam |
| Offline-Transkription | ✅ Gut | ✅ Beste Qualität |
| Akzent/Dialekt | ⚠️ OK | ✅ Sehr gut |
| Mehrsprachig | ⚠️ Ein Modell pro Sprache | ✅ Ein Modell, viele Sprachen |
| CPU-Last | Niedrig (~10%) | Hoch (~30-50%) |
| GPU-Support | Nein | Ja (CUDA/ROCm) |

**Empfehlung:** Verwende VOSK für Live-Streaming, Whisper für Offline-Verarbeitung

## Installation

### 1. Whisper.cpp Binary installieren

Whisper-rs benötigt die whisper.cpp Bibliothek:

```bash
# System-Packages installieren
sudo apt install build-essential pkg-config libclang-dev

# Whisper.cpp clonen und kompilieren
cd ~/.local/share
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp
make

# Bibliothek installieren
sudo cp libwhisper.so /usr/local/lib/
sudo ldconfig
```

### 2. Whisper-Modell herunterladen

Whisper bietet verschiedene Modell-Größen:

```bash
cd ~/.local/share/inox-mix/models
mkdir -p whisper

# Base Model (~142 MB) - Empfohlen für Desktop
wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin

# Small Model (~466 MB) - Höhere Genauigkeit
wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin

# Medium Model (~1.5 GB) - Sehr hohe Genauigkeit
wget https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin
```

### 3. Modell-Pfad konfigurieren

In der inoX-MIX Settings-Sektion:
```
STT → Whisper Model Path: ~/.local/share/inox-mix/models/ggml-base.bin
```

Oder über Config-API:
```javascript
await invoke('set_config', {
  key: 'whisper_model_path',
  value: '~/.local/share/inox-mix/models/ggml-base.bin'
});
```

### 4. STT-Engine aktivieren

```javascript
await invoke('set_stt_engine', { engine: 'whisper' });
```

## Verfügbare Whisper-Modelle

| Modell | Größe | RAM-Nutzung | Genauigkeit | Latenz | GPU-Empfohlen |
|--------|-------|-------------|-------------|--------|---------------|
| tiny | 75 MB | ~390 MB | Niedrig | ~0.5s | Nein |
| base | 142 MB | ~500 MB | Gut | ~1s | Nein |
| small | 466 MB | ~1 GB | Sehr gut | ~2s | Optional |
| medium | 1.5 GB | ~2.6 GB | Excellent | ~4s | Ja |
| large | 2.9 GB | ~4.7 GB | Beste | ~8s | Ja |

Alle Modelle: https://huggingface.co/ggerganov/whisper.cpp/tree/main

## GPU-Beschleunigung (optional)

### CUDA (NVIDIA)

```bash
cd ~/.local/share/whisper.cpp
make clean
WHISPER_CUDA=1 make
sudo cp libwhisper.so /usr/local/lib/
sudo ldconfig
```

### ROCm (AMD)

```bash
cd ~/.local/share/whisper.cpp
make clean
WHISPER_HIPBLAS=1 make
sudo cp libwhisper.so /usr/local/lib/
sudo ldconfig
```

Mit GPU-Beschleunigung:
- **medium** Modell: ~0.8s statt ~4s
- **large** Modell: ~2s statt ~8s

## Sprach-Konfiguration

Whisper unterstützt 99+ Sprachen. Standard ist Auto-Detection.

### Sprache explizit setzen

Via Config:
```bash
# Deutsch erzwingen
"whisper_language": "de"

# Englisch erzwingen
"whisper_language": "en"

# Auto-Detection (Standard)
"whisper_language": null
```

### Unterstützte Sprachen

Wichtigste Sprachen:
- `de` - Deutsch
- `en` - Englisch
- `fr` - Französisch
- `es` - Spanisch
- `it` - Italienisch
- `pt` - Portugiesisch
- `pl` - Polnisch
- `tr` - Türkisch
- `ru` - Russisch
- `zh` - Chinesisch
- `ja` - Japanisch
- `ko` - Koreanisch

Vollständige Liste: https://github.com/openai/whisper#available-models-and-languages

## Performance-Optimierung

### CPU-Threads

Setze `WHISPER_N_THREADS` Umgebungsvariable:

```bash
export WHISPER_N_THREADS=4  # Anzahl CPU-Cores
```

### Memory-Mapping

Für große Modelle (medium/large):

```bash
export WHISPER_USE_MMAP=1
```

### Batch-Size

Für längere Audio-Clips:

```bash
export WHISPER_BATCH_SIZE=512  # Standard: 128
```

## Troubleshooting

### Whisper-Engine startet nicht

1. **Prüfe ob libwhisper.so gefunden wird:**
   ```bash
   ldconfig -p | grep whisper
   ```

2. **Prüfe Modell-Pfad:**
   ```bash
   ls -lh ~/.local/share/inox-mix/models/ggml-base.bin
   ```

3. **Prüfe Log-Ausgabe:**
   ```
   Whisper-Engine wird initialisiert...
   Model-Pfad: /home/user/.local/share/inox-mix/models/ggml-base.bin
   ✓ Whisper-Engine erfolgreich initialisiert
   ```

### Hohe CPU-Last

- Verwende kleinere Modelle (tiny/base statt medium/large)
- Aktiviere GPU-Beschleunigung (CUDA/ROCm)
- Reduziere WHISPER_N_THREADS

### Langsame Transkription

- **Ohne GPU:**
  - tiny: ~0.5s
  - base: ~1s
  - small: ~2s

- **Mit GPU:**
  - small: ~0.3s
  - medium: ~0.8s
  - large: ~2s

- **Tipps:**
  - Verwende GPU-Beschleunigung
  - Wähle kleineres Modell
  - Teile lange Audio-Clips auf

### Niedrige Erkennungsgenauigkeit

- Upgrade auf größeres Modell (base → small → medium)
- Setze Sprache explizit (nicht Auto-Detection)
- Stelle sicher dass Audio mono 16kHz ist
- Verwende Noise-Suppression (DeepFilterNet)

## Vergleich: VOSK vs Whisper

### Wann VOSK verwenden

✅ **Vorteile:**
- Sehr niedrige Latenz (~100-300ms)
- Niedriger RAM-Verbrauch (~150 MB)
- Niedrige CPU-Last (~10%)
- Perfekt für Live-Streaming

⚠️ **Nachteile:**
- Niedrigere Genauigkeit
- Ein Modell pro Sprache
- Schwächer bei Akzenten/Dialekten

### Wann Whisper verwenden

✅ **Vorteile:**
- Höchste Genauigkeit
- Exzellent bei Akzenten/Dialekten
- Multilingual (99+ Sprachen)
- Auto-Language-Detection

⚠️ **Nachteile:**
- Höhere Latenz (~2-5s)
- Höherer RAM-Verbrauch (~500 MB - 5 GB)
- Höhere CPU/GPU-Last
- Nicht für Live-Streaming geeignet

## Hybrid-Setup (Empfohlen)

Nutze beide Engines parallel:

```javascript
// VOSK für Live-Detection
await invoke('set_config', {
  key: 'vosk_model_path',
  value: '~/.local/share/inox-mix/models/vosk-model-small-de-0.15'
});

// Whisper für Offline-Verarbeitung
await invoke('set_config', {
  key: 'whisper_model_path',
  value: '~/.local/share/inox-mix/models/ggml-base.bin'
});

// Engine zur Laufzeit wechseln
await invoke('set_stt_engine', { engine: 'vosk' });    // Live-Stream
await invoke('set_stt_engine', { engine: 'whisper' }); // Offline-Recording
```

**Workflow:**
1. **Live-Streaming:** VOSK (niedrige Latenz)
2. **Offline-Verarbeitung:** Whisper (höchste Genauigkeit)
3. **Beste aus beiden Welten**

## Weitere Ressourcen

- [Whisper.cpp GitHub](https://github.com/ggerganov/whisper.cpp)
- [Whisper-rs Documentation](https://docs.rs/whisper-rs)
- [Whisper Models](https://huggingface.co/ggerganov/whisper.cpp/tree/main)
- [OpenAI Whisper Paper](https://arxiv.org/abs/2212.04356)
