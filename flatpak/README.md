# inoX-MIX Flatpak Package

Dieses Verzeichnis enthält alle Dateien zum Bauen des inoX-MIX Flatpak-Pakets.

## 📋 Voraussetzungen

```bash
# Flatpak installieren
sudo pacman -S flatpak flatpak-builder  # Arch Linux
sudo apt install flatpak flatpak-builder  # Debian/Ubuntu

# Flathub Repository hinzufügen
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Freedesktop SDK installieren
flatpak install flathub org.freedesktop.Platform//23.08
flatpak install flathub org.freedesktop.Sdk//23.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//23.08
flatpak install flathub org.freedesktop.Sdk.Extension.node18//23.08
```

## 🔨 Flatpak Bauen

### Lokaler Build

```bash
cd /home/inox-network/Projekte/inox-mix/flatpak

# Flatpak bauen
flatpak-builder --force-clean --user --install build-dir network.inox.mix.yml

# App ausführen
flatpak run network.inox.mix
```

### Als .flatpak Bundle exportieren

```bash
# Bundle erstellen (für Distribution)
flatpak-builder --force-clean --repo=repo build-dir network.inox.mix.yml
flatpak build-bundle repo inox-mix-0.3.0.flatpak network.inox.mix

# Bundle installieren
flatpak install inox-mix-0.3.0.flatpak
```

## 📦 Flatpak-Struktur

```
flatpak/
├── network.inox.mix.yml           # Flatpak Manifest (Hauptdatei)
├── network.inox.mix.desktop       # Desktop Entry
├── network.inox.mix.metainfo.xml  # AppStream Metadata
├── README.md                      # Diese Datei
└── build.sh                       # Build-Script
```

## 🚀 Veröffentlichung auf Flathub

1. **Fork Flathub Repository:**
   ```bash
   git clone https://github.com/flathub/flathub.git
   cd flathub
   ```

2. **Neuen Branch erstellen:**
   ```bash
   git checkout -b add-inox-mix
   ```

3. **Manifest anpassen:**
   - Ersetze `type: dir` mit `type: archive` (GitHub Release)
   - Füge SHA256-Hash hinzu

4. **Pull Request erstellen:**
   - Zu https://github.com/flathub/flathub
   - PR mit allen Dateien erstellen

## 🔧 Troubleshooting

### Build schlägt fehl

```bash
# Cache löschen
rm -rf build-dir .flatpak-builder

# Rebuild
flatpak-builder --force-clean build-dir network.inox.mix.yml
```

### PipeWire-Zugriff nicht möglich

```bash
# Prüfe ob PipeWire läuft
systemctl --user status pipewire

# Sandbox-Permissions prüfen
flatpak info --show-permissions network.inox.mix

# Extra-Permissions gewähren (falls nötig)
flatpak override --user network.inox.mix --device=all
```

## 📝 Lizenz

MIT License - Copyright (c) 2026 inoX-Network
