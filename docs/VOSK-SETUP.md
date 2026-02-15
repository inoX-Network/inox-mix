# VOSK Speech-to-Text Setup

## Übersicht

inoX-MIX verwendet VOSK für Live-Spracherkennung (Profanity Bleeper). VOSK ist eine offline STT-Engine mit niedriger Latenz (~100-300ms).

## Installation

### 1. VOSK-Modell herunterladen

Deutsches Modell (empfohlen):
```bash
cd ~/.local/share
mkdir -p inox-mix/models
cd inox-mix/models

# Kleines deutsches Modell (~50 MB, schnell)
wget https://alphacephei.com/vosk/models/vosk-model-small-de-0.15.zip
unzip vosk-model-small-de-0.15.zip
rm vosk-model-small-de-0.15.zip
```

Englisches Modell (optional):
```bash
# Kleines englisches Modell (~40 MB)
wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
unzip vosk-model-small-en-us-0.15.zip
rm vosk-model-small-en-us-0.15.zip
```

### 2. Modell-Pfad konfigurieren

In der inoX-MIX Settings-Sektion:
```
STT → VOSK Model Path: ~/.local/share/inox-mix/models/vosk-model-small-de-0.15
```

Oder über Config-API:
```javascript
await invoke('set_config', {
  key: 'vosk_model_path',
  value: '~/.local/share/inox-mix/models/vosk-model-small-de-0.15'
});
```

### 3. STT-Engine aktivieren

```javascript
await invoke('set_stt_engine', { engine: 'vosk' });
```

## Verfügbare VOSK-Modelle

| Modell | Sprache | Größe | Genauigkeit | Latenz |
|--------|---------|-------|-------------|---------|
| vosk-model-small-de-0.15 | Deutsch | 50 MB | Gut | ~100ms |
| vosk-model-de-0.21 | Deutsch | 1.8 GB | Sehr gut | ~200ms |
| vosk-model-small-en-us-0.15 | Englisch | 40 MB | Gut | ~100ms |
| vosk-model-en-us-0.22 | Englisch | 1.8 GB | Sehr gut | ~200ms |

Alle Modelle: https://alphacephei.com/vosk/models

## Profanity Word Management

### Wörter hinzufügen

```javascript
await invoke('add_profanity_word', {
  word: 'badword',
  category: 'schimpf',  // schimpf | beleid | rass | custom
  language: 'de'
});
```

### Wörter entfernen

```javascript
await invoke('remove_profanity_word', {
  word: 'badword'
});
```

### Wörter abrufen

```javascript
// Alle Wörter
const words = await invoke('get_profanity_words', {
  category: null,
  language: null
});

// Nur deutsche Schimpfwörter
const german_swears = await invoke('get_profanity_words', {
  category: 'schimpf',
  language: 'de'
});
```

## Kategorien

- **schimpf**: Allgemeine Schimpfwörter
- **beleid**: Beleidigungen
- **rass**: Rassistische/diskriminierende Ausdrücke
- **custom**: Benutzerdefinierte Wörter

## Troubleshooting

### VOSK-Engine startet nicht

1. Prüfe ob Modell-Pfad korrekt ist:
   ```bash
   ls -lh ~/.local/share/inox-mix/models/vosk-model-small-de-0.15/
   ```

2. Prüfe Log-Ausgabe:
   ```
   VOSK-Engine wird initialisiert...
   Model-Pfad: /home/user/.local/share/inox-mix/models/vosk-model-small-de-0.15
   ✓ VOSK-Engine erfolgreich initialisiert
   ```

3. Falls "Modell konnte nicht geladen werden":
   - Pfad ungültig → Korrigiere vosk_model_path in Config
   - Modell beschädigt → Erneut herunterladen
   - Berechtigungen → `chmod -R 755 ~/.local/share/inox-mix/models/`

### Hohe CPU-Last

- Verwende kleinere Modelle (small statt large)
- Reduziere Sample-Rate (16kHz empfohlen)
- Deaktiviere STT wenn nicht benötigt

### Erkennungsgenauigkeit niedrig

- Upgrade auf größeres Modell (vosk-model-de-0.21)
- Stelle sicher dass Mikrofon-Input klar ist
- Reduziere Hintergrundgeräusche
- Verwende Noise-Suppression (DeepFilterNet)

## Performance

| Modell | RAM-Nutzung | CPU-Last | Latenz | GPU-Support |
|--------|-------------|----------|---------|-------------|
| small-de | ~150 MB | ~10% | ~100ms | Nein |
| de-0.21 | ~2 GB | ~20% | ~200ms | Ja (optional) |

## Alternative: Whisper

Für höchste Genauigkeit (aber höhere Latenz ~2-5s) kann Whisper verwendet werden:

```javascript
await invoke('set_stt_engine', { engine: 'whisper' });
```

**Hinweis:** Whisper ist noch nicht implementiert (Task #7).
