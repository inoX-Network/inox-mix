# inoX-MIX AUR Package

Dieses Verzeichnis enthält das PKGBUILD für das Arch User Repository (AUR).

## 📋 Installation für End-Benutzer

### Via AUR Helper (empfohlen)

```bash
# Mit yay
yay -S inox-mix

# Mit paru
paru -S inox-mix

# Mit trizen
trizen -S inox-mix
```

### Manuell via makepkg

```bash
# Repository klonen
git clone https://aur.archlinux.org/inox-mix.git
cd inox-mix

# Bauen und installieren
makepkg -si
```

## 🔨 Für Maintainer

### Lokalen Test-Build

```bash
cd /home/inox-network/Projekte/inox-mix/aur

# PKGBUILD testen
makepkg -f

# Paket installieren
sudo pacman -U inox-mix-0.3.0-1-x86_64.pkg.tar.zst
```

### .SRCINFO generieren

```bash
cd /home/inox-network/Projekte/inox-mix/aur

# .SRCINFO erstellen/aktualisieren
makepkg --printsrcinfo > .SRCINFO
```

### AUR Upload

```bash
# Erstes Mal: AUR Repository erstellen
ssh aur@aur.archlinux.org setup-repo inox-mix

# Repository klonen
git clone ssh://aur@aur.archlinux.org/inox-mix.git aur-repo
cd aur-repo

# Dateien kopieren
cp ../PKGBUILD .
cp ../inox-mix.install .
makepkg --printsrcinfo > .SRCINFO

# Commit und Push
git add PKGBUILD inox-mix.install .SRCINFO
git commit -m "Initial commit: inox-mix 0.3.0"
git push origin master
```

### Update auf neue Version

```bash
cd aur-repo

# PKGBUILD aktualisieren (pkgver und sha256sum)
vim PKGBUILD

# .SRCINFO neu generieren
makepkg --printsrcinfo > .SRCINFO

# Testen
makepkg -f

# Commit und Push
git add PKGBUILD .SRCINFO
git commit -m "Update to 0.3.1"
git push
```

## 📦 Dateien

```
aur/
├── PKGBUILD           # Build-Anweisungen
├── inox-mix.install   # Post-Install Hooks
├── .SRCINFO           # Metadata (generiert)
├── README.md          # Diese Datei
└── test-build.sh      # Test-Script
```

## 🔧 Dependencies

### Build-Dependencies (makedepends)
- `rust` - Rust Compiler
- `cargo` - Rust Package Manager
- `nodejs` - Node.js Runtime
- `npm` - Node Package Manager
- `git` - Version Control

### Runtime-Dependencies (depends)
- `pipewire` - Audio-Server
- `webkit2gtk` - WebView Engine
- `gtk3` - GTK Toolkit
- `libsoup` - HTTP Library
- `javascriptcore` - JavaScript Engine

### Optional-Dependencies (optdepends)
- `pipewire-pulse` - PulseAudio-Kompatibilität
- `wireplumber` - PipeWire Session Manager

## ✅ Checklist für AUR Submission

- [ ] PKGBUILD validiert (`namcap PKGBUILD`)
- [ ] Paket gebaut (`makepkg -f`)
- [ ] Paket validiert (`namcap inox-mix-*.pkg.tar.zst`)
- [ ] .SRCINFO generiert (`makepkg --printsrcinfo > .SRCINFO`)
- [ ] SHA256-Checksumme aktualisiert
- [ ] LICENSE Datei vorhanden
- [ ] Desktop-Integration getestet
- [ ] PipeWire-Zugriff funktioniert
- [ ] AUR Account erstellt
- [ ] SSH Key bei AUR hochgeladen

## 🆘 Troubleshooting

### Build schlägt fehl

```bash
# Dependencies prüfen
pacman -Q rust cargo nodejs npm

# Cache löschen
rm -rf src/ pkg/ *.pkg.tar.zst

# Rebuild
makepkg -Cf
```

### Checksum-Fehler

```bash
# Neue Checksum generieren
makepkg -g >> PKGBUILD

# Dann manuell die alte sha256sum Zeile entfernen
```

### Test-Installation

```bash
# In Clean-Chroot bauen (wie AUR-Builder es macht)
extra-x86_64-build
```

## 📝 Lizenz

MIT License - Copyright (c) 2026 inoX-Network
