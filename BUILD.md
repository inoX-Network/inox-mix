# inoX-MIX Build & Packaging Guide

## Voraussetzungen

### System-Dependencies (Ubuntu/Debian)

```bash
# Rust Toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Node.js & npm (v18+)
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Tauri Dependencies
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# PipeWire Development Headers
sudo apt-get install -y \
    libpipewire-0.3-dev \
    pipewire \
    wireplumber

# Audio Libraries
sudo apt-get install -y \
    libasound2-dev \
    libpulse-dev
```

### Arch Linux

```bash
# Rust Toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Dependencies
sudo pacman -S --needed \
    webkit2gtk-4.1 \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    libappindicator-gtk3 \
    librsvg \
    pipewire \
    wireplumber \
    nodejs \
    npm
```

---

## Development Build

### 1. Frontend installieren

```bash
npm install
```

### 2. Development-Server starten

```bash
npm run tauri dev
```

Der Dev-Server startet auf `http://localhost:1420` mit Hot-Reload.

---

## Production Build

### 1. Dependencies installieren

```bash
npm install
cd src-tauri
cargo fetch
cd ..
```

### 2. Production-Build erstellen

```bash
npm run tauri build
```

**Output:**
- `src-tauri/target/release/inox-mix` — Binary
- `src-tauri/target/release/bundle/appimage/inox-mix_0.3.0_amd64.AppImage`
- `src-tauri/target/release/bundle/deb/inox-mix_0.3.0_amd64.deb`

---

## Manuelle Builds

### AppImage

```bash
cd src-tauri
cargo tauri build --bundles appimage
```

Output: `target/release/bundle/appimage/`

### Debian Package

```bash
cd src-tauri
cargo tauri build --bundles deb
```

Output: `target/release/bundle/deb/`

---

## Build-Optimierungen

### Release-Profile (Cargo.toml)

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = 'abort'
```

### Bundle-Size reduzieren

```bash
# UPX Kompression für Binary
sudo apt-get install upx
upx --best --lzma target/release/inox-mix

# AppImage optimieren
ARCH=x86_64 appimagetool --comp gzip target/appimage/
```

---

## CI/CD GitHub Actions

Automatische Builds bei Git-Tags (`v*`):

1. Push ein Tag: `git tag v0.3.0 && git push origin v0.3.0`
2. GitHub Actions erstellt automatisch:
   - AppImage (x86_64)
   - .deb Package
   - GitHub Release mit Binaries

Siehe `.github/workflows/release.yml`

---

## Packaging-Metadaten

### Desktop Entry

Erstellt automatisch: `/usr/share/applications/inox-mix.desktop`

```ini
[Desktop Entry]
Name=inoX-MIX
Comment=Professional Audio Mixer for Linux Streamers
Exec=inox-mix
Icon=inox-mix
Type=Application
Categories=AudioVideo;Audio;Mixer;
```

### AppStream Metadata

Für Software-Center: `/usr/share/metainfo/network.inox.mix.metainfo.xml`

---

## Troubleshooting

### Build-Fehler: "PipeWire not found"

```bash
sudo apt-get install libpipewire-0.3-dev
```

### Build-Fehler: "webkit2gtk not found"

```bash
sudo apt-get install libwebkit2gtk-4.1-dev
```

### AppImage startet nicht

```bash
# FUSE installieren
sudo apt-get install fuse libfuse2

# AppImage ausführbar machen
chmod +x inox-mix_0.3.0_amd64.AppImage

# Direkt extrahieren und ausführen
./inox-mix_0.3.0_amd64.AppImage --appimage-extract
./squashfs-root/AppRun
```

### .deb Installation-Fehler

```bash
# Dependencies manuell installieren
sudo apt-get install -f

# Dann erneut versuchen
sudo dpkg -i inox-mix_0.3.0_amd64.deb
```

---

## Update-System testen

### Lokales Update-Testing

1. Build erstellen: `npm run tauri build`
2. `latest.json` erstellen:

```json
{
  "version": "v0.3.0",
  "notes": "Neue Features: ...",
  "pub_date": "2026-02-15T12:00:00Z",
  "platforms": {
    "linux-x86_64": {
      "signature": "",
      "url": "https://github.com/inox-network/inox-mix/releases/download/v0.3.0/inox-mix_0.3.0_amd64.AppImage"
    }
  }
}
```

3. Lokalen HTTP-Server starten:

```bash
python3 -m http.server 8000
```

4. `tauri.conf.json` anpassen:

```json
"endpoints": ["http://localhost:8000/latest.json"]
```

---

## Distribution

### Flatpak (TODO)

```bash
flatpak-builder --repo=repo build-dir network.inox.mix.yml
flatpak build-bundle repo inox-mix.flatpak network.inox.mix
```

### AUR Package (TODO)

PKGBUILD für Arch User Repository erstellen.

---

## Performance-Testing

```bash
# Binary-Size prüfen
ls -lh src-tauri/target/release/inox-mix

# Startup-Zeit messen
time ./src-tauri/target/release/inox-mix --version

# Memory-Usage profilen
valgrind --tool=massif ./src-tauri/target/release/inox-mix
```

---

## Checkliste für Release

- [ ] Version in `Cargo.toml` erhöhen
- [ ] Version in `package.json` erhöhen
- [ ] Version in `tauri.conf.json` erhöhen
- [ ] CHANGELOG.md aktualisieren
- [ ] Git-Tag erstellen: `git tag v0.3.0`
- [ ] Tag pushen: `git push origin v0.3.0`
- [ ] GitHub Actions Workflow abwarten
- [ ] Release-Notes in GitHub Release hinzufügen
- [ ] Binaries lokal testen
- [ ] Update-System testen (alte Version → neue Version)
