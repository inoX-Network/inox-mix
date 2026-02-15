# 🚀 inoX-MIX Flatpak - Quick Start

## Schnellstart (für Entwickler)

```bash
# 1. Ins Flatpak-Verzeichnis wechseln
cd /home/inox-network/Projekte/inox-mix/flatpak

# 2. Build-Script ausführen
./build.sh

# 3. App starten
flatpak run network.inox.mix
```

## Für End-Benutzer

### Installation via .flatpak Bundle

```bash
# Bundle herunterladen (wird später auf GitHub Releases verfügbar sein)
wget https://github.com/inox-network/inox-mix/releases/download/v0.3.0/inox-mix-0.3.0.flatpak

# Installieren
flatpak install inox-mix-0.3.0.flatpak

# Starten
flatpak run network.inox.mix
```

### Installation via Flathub (Zukünftig)

```bash
# Nach Veröffentlichung auf Flathub:
flatpak install flathub network.inox.mix
flatpak run network.inox.mix
```

## 🔧 Debugging

### App-Logs anzeigen

```bash
flatpak run network.inox.mix 2>&1 | tee inox-mix.log
```

### In Sandbox Shell wechseln

```bash
flatpak run --command=sh network.inox.mix
```

### Permissions prüfen

```bash
flatpak info --show-permissions network.inox.mix
```

### Extra Audio-Permissions (falls nötig)

```bash
flatpak override --user network.inox.mix --device=all --socket=pulseaudio
```

## 📦 Bundle erstellen (für Maintainer)

```bash
cd flatpak/
./build.sh
# Wähle Option 2: Bundle erstellen
```

Das erzeugt `inox-mix-0.3.0.flatpak` im flatpak-Verzeichnis.

## 🌐 Flathub Submission Checklist

- [ ] Manifest validiert (`flatpak-builder --stop-at=inox-mix build-dir network.inox.mix.yml`)
- [ ] Desktop File validiert (`desktop-file-validate network.inox.mix.desktop`)
- [ ] AppStream Metadata validiert (`appstream-util validate network.inox.mix.metainfo.xml`)
- [ ] Screenshots hinzugefügt (1280x720 oder 1920x1080)
- [ ] Icon in allen Größen (64x64, 128x128, 256x256, 512x512)
- [ ] Lizenz korrekt (MIT)
- [ ] GitHub Release erstellt mit Source-Tarball
- [ ] Fork von flathub/flathub erstellt
- [ ] Pull Request mit allen Dateien erstellt

## 🆘 Support

Bei Problemen:
- GitHub Issues: https://github.com/inox-network/inox-mix/issues
- Wiki: https://github.com/inox-network/inox-mix/wiki
