# inoX-MIX

**Professioneller Desktop Audio-Mixer für Linux Streamer**

![Version](https://img.shields.io/badge/version-0.3.0-cyan)
![License](https://img.shields.io/badge/license-MIT-orange)
![Platform](https://img.shields.io/badge/platform-Linux-blue)

---

## 🎵 Features

### Core Mixer
- ✅ **Input Strips** — Hardware (Cyan) und Virtual (Orange) Audio-Quellen
- ✅ **Output Buses** — 4 Busse (A1 Speakers, A2 Headset, B1 Stream, B2 VoIP)
- ✅ **Routing Matrix** — Flexibles Audio-Routing zwischen Quellen und Zielen
- ✅ **Master Section** — Master-Fader, Limiter, DIM, MONO, Talkback

### Signal Processing
- ✅ **FX-Chain** — HPF, Gate, De-Esser, EQ, Compressor, Limiter, Auto-Gain
- ✅ **Voice FX** — 7 Presets (Robot, Vader, Chipmunk, Megaphone, Echo, Radio)
- ✅ **Ducking** — Automatisches Musik-Ducking bei Sprache (Sidechain)
- ✅ **Denoise** — AI-basierte Geräuschunterdrückung (DeepFilterNet/RNNoise)

### Streamer-Tools
- ✅ **Soundboard** — Hot-Key gesteuerte Sound-Pads
- ✅ **Bleeper** — Profanity-Filter mit 5 Modi (Beep, Mute, Noise, Reverse, Custom)
- ✅ **Recording** — Multi-Track Aufnahme in FLAC/WAV
- ✅ **Scenes** — Speichern & Laden von kompletten Mixer-States

### System
- ✅ **PipeWire Integration** — Native Linux Audio mit niedriger Latenz
- ✅ **Echtzeit-Metering** — Peak/RMS VU-Meter (60fps)
- ✅ **Quick Calibrate** — Automatische Mikrofon-Kalibrierung
- ✅ **Auto-Update** — GitHub Releases + VPS Fallback
- ✅ **App-Mixer** — Per-Application Audio-Routing

---

## 📦 Installation

### AppImage (Universal)

```bash
# Download latest release
wget https://github.com/inox-network/inox-mix/releases/latest/download/inox-mix-0.3.0-x86_64.AppImage

# Ausführbar machen
chmod +x inox-mix-0.3.0-x86_64.AppImage

# Starten
./inox-mix-0.3.0-x86_64.AppImage
```

### Debian/Ubuntu (.deb)

```bash
# Download
wget https://github.com/inox-network/inox-mix/releases/latest/download/inox-mix-0.3.0-amd64.deb

# Installieren
sudo dpkg -i inox-mix-0.3.0-amd64.deb
sudo apt-get install -f  # Dependencies nachinstallieren

# Starten
inox-mix
```

### Arch Linux (AUR)

```bash
# TODO: AUR Package wird erstellt
yay -S inox-mix
```

---

## 🚀 Quick Start

### 1. PipeWire Setup

inoX-MIX benötigt PipeWire:

```bash
# PipeWire installieren (Ubuntu/Debian)
sudo apt-get install pipewire wireplumber pipewire-pulse

# PipeWire starten
systemctl --user start pipewire pipewire-pulse wireplumber

# Status prüfen
systemctl --user status pipewire
```

### 2. Audio-Geräte konfigurieren

1. **Einstellungen** öffnen (⚙️ Icon)
2. **Audio** → Geräte auswählen
3. **Sample Rate** einstellen (48000 Hz empfohlen)
4. **Buffer Size** anpassen (256 für niedrige Latenz)

### 3. Quick Calibrate

1. Mikrofon auswählen
2. **Quick Calibrate** Button klicken
3. 10 Sekunden normal sprechen
4. Empfohlene Einstellungen übernehmen

### 4. Routing einrichten

1. **Routing-Matrix** öffnen
2. Quellen zu Bussen routen:
   - Mikrofon → A1 (Speakers) + B1 (Stream)
   - Browser → A1 (Speakers)
   - Spotify → A1 (Speakers)

---

## 🎨 UI-Übersicht

```
┌─────────────────────────────────────────────────────────┐
│  Header: Recording • Scenes • Updates                  │
├──────────┬──────────────────────────────────┬──────────┤
│  Mixer   │  Output Buses                    │ Streamer │
│          │                                  │          │
│ ┌──────┐ │ ┌──────┐ ┌──────┐ ┌──────┐ ┌──┐ │ Voice FX │
│ │ MIC  │ │ │  A1  │ │  A2  │ │  B1  │ │B2│ │ Ducking  │
│ │ VU   │ │ │ VU   │ │ VU   │ │ VU   │ │VU│ │ Bleeper  │
│ │ VOL  │ │ │ VOL  │ │ VOL  │ │ VOL  │ │V │ │ Sound-   │
│ │ FX   │ │ │ MUTE │ │ MUTE │ │ MUTE │ │M │ │  board   │
│ └──────┘ │ └──────┘ └──────┘ └──────┘ └──┘ │          │
├──────────┴──────────────────────────────────┴──────────┤
│  FX-Chain: HPF • Gate • De-Esser • EQ • Comp • Limiter │
└─────────────────────────────────────────────────────────┘
```

---

## 🔧 Technische Details

### Technologie-Stack

| Komponente | Technologie |
|-----------|-------------|
| Runtime | Tauri 2.x (Rust + WebView) |
| Frontend | React 18 + TypeScript + Vite |
| Styling | Tailwind CSS + Oxanium Font |
| Backend | Rust (PipeWire Bindings) |
| Audio | PipeWire (pw-cli, pw-link, pw-loopback) |
| Database | SQLite (Config/Presets) |
| Updates | GitHub Releases + VPS Fallback |

### Audio-Spezifikationen

- **Sample Rate**: 48000 Hz (Standard)
- **Buffer Size**: 256 Samples (Standard)
- **Bit Depth**: 32-bit float (intern)
- **Latenz**: < 10ms (abhängig von Buffer-Size)
- **Metering**: Peak + RMS @ 60fps
- **dB-Skala**: -60dB bis +10dB

### Farbschema

- **Cyan (#00e5ff)** — Hardware/A-Busse/Master
- **Orange (#ff8c00)** — Virtual/B-Busse/Stream
- **Rot (#ff1744)** — Fehler/Mute/Clip/REC
- **Grün (#4caf50)** — Status OK

---

## 📚 Dokumentation

- [BUILD.md](BUILD.md) — Build & Packaging Guide
- [CHANGELOG.md](CHANGELOG.md) — Version History
- [CLAUDE.md](CLAUDE.md) — Development Instructions
- [specs/](specs/) — Feature Specifications

---

## 🛠️ Development

### Voraussetzungen

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js 18+
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Dependencies
sudo apt-get install -y libwebkit2gtk-4.1-dev libpipewire-0.3-dev
```

### Dev-Server starten

```bash
npm install
npm run tauri dev
```

### Build erstellen

```bash
npm run tauri build
```

### Tests ausführen

```bash
# Backend Tests
cd src-tauri
cargo test

# Frontend Tests (TODO)
npm test
```

---

## 🐛 Troubleshooting

### Kein Audio-Signal

1. PipeWire läuft: `systemctl --user status pipewire`
2. Richtiges Eingabegerät in Einstellungen
3. Mikro nicht gemuted
4. Gate-Threshold nicht zu hoch → **Quick Calibrate**

### Hohe Latenz

1. Buffer-Size reduzieren (128 statt 256)
2. Nicht benötigte FX-Module deaktivieren
3. PipeWire Quantum prüfen: `pw-metadata -n settings`

### Update funktioniert nicht

1. Internet-Verbindung prüfen
2. GitHub Releases erreichbar: `curl https://github.com/inox-network/inox-mix/releases`
3. Manuell updaten: Neue Version herunterladen

---

## 🤝 Contributing

Contributions sind willkommen! Bitte:

1. Issue erstellen für Features/Bugs
2. Fork erstellen
3. Feature-Branch erstellen (`git checkout -b feature/amazing-feature`)
4. Committen (`git commit -m 'Add amazing feature'`)
5. Pushen (`git push origin feature/amazing-feature`)
6. Pull Request erstellen

---

## 📄 Lizenz

MIT License - siehe [LICENSE](LICENSE)

---

## 🙏 Credits

- **DeepFilterNet** — AI Noise Reduction
- **PipeWire** — Modern Linux Audio
- **Tauri** — Desktop Framework
- **Oxanium Font** — Google Fonts

---

## 📬 Kontakt

- **GitHub**: https://github.com/inox-network/inox-mix
- **Issues**: https://github.com/inox-network/inox-mix/issues
- **Website**: https://inox-network.de

---

**Made with ❤️ by inoX-Network**
