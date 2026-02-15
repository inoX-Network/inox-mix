# 🚀 inoX-MIX v0.3.0 Release Checklist

Dieses Dokument beschreibt alle Schritte für einen erfolgreichen Release von inoX-MIX.

**Datum:** 2026-02-15
**Version:** 0.3.0
**Maintainer:** inoX-Network

---

## 📋 Phase 1: Pre-Release Checks

### Code-Qualität

- [ ] Alle Tests laufen durch
  ```bash
  cd src-tauri
  cargo test --release
  ```

- [ ] Keine Compiler-Warnungen (außer unused code)
  ```bash
  cargo clippy --release
  ```

- [ ] Code formatiert
  ```bash
  cargo fmt --check
  ```

- [ ] Frontend baut ohne Fehler
  ```bash
  npm run build
  ```

### Dokumentation

- [ ] README.md aktualisiert
  - [ ] Version-Nummer korrekt
  - [ ] Features-Liste aktuell
  - [ ] Screenshots aktuell (falls vorhanden)
  - [ ] Installation-Anweisungen vollständig

- [ ] CHANGELOG.md erstellt/aktualisiert
  - [ ] Alle neuen Features dokumentiert
  - [ ] Bugfixes aufgelistet
  - [ ] Breaking Changes erwähnt

- [ ] LICENSE vorhanden und korrekt
  - [x] MIT License (bereits erstellt)

### Dependencies & Security

- [ ] Cargo.lock committet
  ```bash
  git add src-tauri/Cargo.lock
  git commit -m "chore: update Cargo.lock"
  ```

- [ ] npm audit check
  ```bash
  npm audit
  ```

- [ ] cargo audit (optional)
  ```bash
  cargo install cargo-audit
  cargo audit
  ```

### Version-Nummern

- [ ] **src-tauri/Cargo.toml** → version = "0.3.0"
- [ ] **src-tauri/tauri.conf.json** → version = "0.3.0"
- [ ] **package.json** → version = "0.3.0"
- [ ] **flatpak/network.inox.mix.yml** → pkgver = 0.3.0
- [ ] **aur/PKGBUILD** → pkgver = 0.3.0

---

## 📦 Phase 2: Build & Test

### Lokale Builds

- [ ] **Release Build** erfolgreich
  ```bash
  cd src-tauri
  cargo build --release
  ```

- [ ] **AppImage Build** (via Tauri)
  ```bash
  npm run tauri build -- --target appimage
  ```

- [ ] **DEB Package Build**
  ```bash
  npm run tauri build -- --target deb
  ```

- [ ] **Binary funktioniert**
  ```bash
  ./src-tauri/target/release/inox-mix
  ```

### Flatpak Build

- [ ] Flatpak baut erfolgreich
  ```bash
  cd flatpak
  ./build.sh
  # Option 1 wählen
  ```

- [ ] Flatpak läuft
  ```bash
  flatpak run network.inox.mix
  ```

- [ ] PipeWire-Zugriff funktioniert
- [ ] Audio-Capture funktioniert
- [ ] Ducking funktioniert
- [ ] Bleeper funktioniert

### AUR Test-Build

- [ ] PKGBUILD baut erfolgreich
  ```bash
  cd aur
  ./test-build.sh
  # Option 1 wählen
  ```

- [ ] Paket validiert
  ```bash
  namcap PKGBUILD
  namcap inox-mix-*.pkg.tar.zst
  ```

- [ ] Test-Installation funktioniert
  ```bash
  sudo pacman -U inox-mix-*.pkg.tar.zst
  inox-mix
  ```

---

## 🌐 Phase 3: Git & GitHub

### Git Vorbereitung

- [ ] Alle Änderungen committet
  ```bash
  git status
  # Sollte "nothing to commit" zeigen
  ```

- [ ] Main-Branch ist aktuell
  ```bash
  git checkout main
  git pull origin main
  ```

- [ ] Changelog committet
  ```bash
  git add CHANGELOG.md
  git commit -m "docs: update changelog for v0.3.0"
  ```

### Git Tag erstellen

- [ ] Version-Tag erstellen
  ```bash
  git tag -a v0.3.0 -m "Release v0.3.0 - Audio-Capture Integration

  Features:
  - CPAL audio capture integration
  - Sidechain ducking implementation
  - Profanity bleeper (5 modes)
  - Flatpak package support
  - AUR package support

  Technical:
  - Thread-safe CPAL architecture
  - Real-time VU metering (60fps)
  - PipeWire integration
  "
  ```

- [ ] Tag pushen
  ```bash
  git push origin v0.3.0
  git push origin main
  ```

### GitHub Release

- [ ] Zu GitHub Releases gehen
  ```
  https://github.com/inox-network/inox-mix/releases/new
  ```

- [ ] Release erstellen
  - [ ] Tag: v0.3.0
  - [ ] Title: "inoX-MIX v0.3.0 - Audio-Capture Integration"
  - [ ] Description aus Template (siehe unten)
  - [ ] "Set as the latest release" aktiviert

- [ ] Assets hochladen
  - [ ] `inox-mix-0.3.0-x86_64.AppImage`
  - [ ] `inox-mix-0.3.0-amd64.deb`
  - [ ] `inox-mix-0.3.0.flatpak` (optional)
  - [ ] Source-Code (auto-generiert von GitHub)

- [ ] SHA256-Checksums erstellen
  ```bash
  sha256sum inox-mix-0.3.0-x86_64.AppImage > checksums.txt
  sha256sum inox-mix-0.3.0-amd64.deb >> checksums.txt
  sha256sum v0.3.0.tar.gz >> checksums.txt
  ```

- [ ] checksums.txt hochladen

---

## 📦 Phase 4: Flatpak Distribution

### Flatpak Bundle erstellen

- [ ] Bundle bauen
  ```bash
  cd flatpak
  ./build.sh
  # Option 2 wählen
  ```

- [ ] Bundle testen
  ```bash
  flatpak install inox-mix-0.3.0.flatpak
  flatpak run network.inox.mix
  ```

### Flathub Submission (Optional, später)

- [ ] Flathub-Repository forken
  ```bash
  git clone https://github.com/flathub/flathub.git
  ```

- [ ] Manifest anpassen
  - [ ] `type: dir` → `type: archive`
  - [ ] GitHub-Release-URL eintragen
  - [ ] SHA256-Hash hinzufügen

- [ ] Screenshots erstellen
  - [ ] Hauptansicht (1920x1080 oder 1280x720)
  - [ ] In `screenshots/` ablegen

- [ ] Pull Request erstellen
  - [ ] Zu https://github.com/flathub/flathub
  - [ ] Titel: "Add inoX-MIX"
  - [ ] Beschreibung aus Template

---

## 🏛️ Phase 5: AUR Upload

### SHA256-Checksum aktualisieren

- [ ] Source-Tarball herunterladen
  ```bash
  cd aur
  wget https://github.com/inox-network/inox-mix/archive/v0.3.0.tar.gz
  ```

- [ ] SHA256 berechnen
  ```bash
  sha256sum v0.3.0.tar.gz
  ```

- [ ] In PKGBUILD eintragen
  ```bash
  vim PKGBUILD
  # sha256sums=('HIER_DER_HASH') statt 'SKIP'
  ```

### .SRCINFO aktualisieren

- [ ] .SRCINFO generieren
  ```bash
  makepkg --printsrcinfo > .SRCINFO
  ```

### Finaler Test

- [ ] Nochmal Test-Build
  ```bash
  ./test-build.sh
  ```

- [ ] Paket validieren
  ```bash
  namcap PKGBUILD
  namcap inox-mix-*.pkg.tar.zst
  ```

### AUR Upload

- [ ] AUR-Konto erstellt
  - [ ] Account: https://aur.archlinux.org/register
  - [ ] SSH-Key hochgeladen

- [ ] Repository erstellen (nur beim ersten Mal)
  ```bash
  ssh aur@aur.archlinux.org setup-repo inox-mix
  ```

- [ ] Repository klonen
  ```bash
  git clone ssh://aur@aur.archlinux.org/inox-mix.git aur-upload
  cd aur-upload
  ```

- [ ] Dateien kopieren
  ```bash
  cp ../PKGBUILD .
  cp ../inox-mix.install .
  cp ../.SRCINFO .
  ```

- [ ] Commit und Push
  ```bash
  git add PKGBUILD inox-mix.install .SRCINFO
  git commit -m "Initial release: inox-mix 0.3.0"
  git push origin master
  ```

- [ ] AUR-Seite prüfen
  ```
  https://aur.archlinux.org/packages/inox-mix
  ```

- [ ] Installation testen
  ```bash
  yay -S inox-mix
  ```

---

## 📢 Phase 6: Kommunikation & Promotion

### Ankündigung

- [ ] Release-Notes auf Website (falls vorhanden)

- [ ] Social Media Posts vorbereiten
  - [ ] Twitter/X
  - [ ] Mastodon
  - [ ] Reddit (r/linux, r/linuxaudio, r/archlinux)
  - [ ] Discord/Matrix Communities

- [ ] Blog-Post (optional)
  - [ ] Features beschreiben
  - [ ] Screenshots/GIFs
  - [ ] Installation-Anleitung

### Benachrichtigungen

- [ ] GitHub-Follower (automatisch)
- [ ] Mailing-Liste (falls vorhanden)
- [ ] IRC/Matrix-Channel

---

## ✅ Phase 7: Post-Release

### Monitoring

- [ ] GitHub Issues beobachten (erste 48h)
- [ ] AUR Comments prüfen
- [ ] Crash-Reports analysieren (falls vorhanden)

### Dokumentation

- [ ] Release im Changelog dokumentiert
- [ ] Version auf Website aktualisiert (falls vorhanden)
- [ ] Wiki aktualisiert (falls vorhanden)

### Nächste Version planen

- [ ] GitHub Milestones für v0.4.0 erstellen
- [ ] Feature-Requests priorisieren
- [ ] Roadmap aktualisieren

---

## 📝 Templates

### GitHub Release Description Template

```markdown
# 🎉 inoX-MIX v0.3.0 - Audio-Capture Integration

Professional Audio Mixer for Linux Streamers with PipeWire integration

## ✨ New Features

- **CPAL Audio-Capture**: Real audio input from microphones (no more simulation!)
- **Sidechain Ducking**: Music automatically quiets when you speak
- **Profanity Bleeper**: 5 modes (Beep, Mute, Noise, Reverse, Custom)
- **Flatpak Support**: Universal Linux package
- **AUR Support**: Easy installation for Arch Linux users

## 🔧 Technical Improvements

- Thread-safe CPAL architecture
- Real-time VU metering (60fps)
- PipeWire integration with dynamic node discovery
- Envelope-follower for ducking
- Comprehensive test suite (15+ unit tests)

## 📦 Installation

### Flatpak (All Distributions)
```bash
flatpak install inox-mix-0.3.0.flatpak
flatpak run network.inox.mix
```

### Arch Linux (AUR)
```bash
yay -S inox-mix
```

### AppImage (Portable)
```bash
chmod +x inox-mix-0.3.0-x86_64.AppImage
./inox-mix-0.3.0-x86_64.AppImage
```

### Debian/Ubuntu
```bash
sudo dpkg -i inox-mix-0.3.0-amd64.deb
```

## 🐛 Bug Reports

Report issues at: https://github.com/inox-network/inox-mix/issues

## 📄 Checksums

See `checksums.txt` for SHA256 hashes of all downloads.

## 🙏 Contributors

Thanks to everyone who contributed to this release!

---

**Full Changelog**: https://github.com/inox-network/inox-mix/compare/v0.2.0...v0.3.0
```

### Flathub PR Description Template

```markdown
# Add inoX-MIX

Professional Audio Mixer for Linux Streamers

**Homepage**: https://github.com/inox-network/inox-mix
**License**: MIT
**Category**: AudioVideo

## Description

inoX-MIX is a professional desktop audio mixer specifically designed for Linux streamers,
podcasters, and content creators.

Features:
- PipeWire integration
- Real-time VU metering
- Sidechain ducking
- Profanity bleeper
- 8-stage FX chain
- Soundboard support

## Checklist

- [x] Manifest follows Flathub guidelines
- [x] AppStream metadata included
- [x] Desktop file included
- [x] Icons in multiple sizes
- [x] License is MIT
- [x] Tested locally

## Testing

```bash
flatpak-builder --user --install build-dir network.inox.mix.yml
flatpak run network.inox.mix
```

Works as expected on Fedora 39 and Arch Linux.
```

---

## 🎯 Success Criteria

Release ist erfolgreich wenn:

- [ ] GitHub Release ist live
- [ ] Mindestens 1 Download in 24h
- [ ] Keine kritischen Bugs gemeldet
- [ ] AUR-Paket installierbar via `yay -S inox-mix`
- [ ] Flatpak läuft auf mind. 2 verschiedenen Distros

---

## 🆘 Rollback-Plan

Falls kritische Bugs gefunden werden:

1. **GitHub Release als "Pre-release" markieren**
   - Edit Release → "Set as a pre-release"

2. **AUR-Paket als "out of date" markieren**
   - Auf AUR-Seite: "Flag Package Out-Of-Date"

3. **Hotfix-Release vorbereiten**
   - Branch: `hotfix/v0.3.1`
   - Schneller Fix
   - Neuer Release v0.3.1

4. **Communication**
   - GitHub Issue mit Warnung
   - Update-Notification

---

**Viel Erfolg beim Release! 🚀**
