# 📦 AUR Submission Guide für inoX-MIX

Diese Anleitung beschreibt, wie inoX-MIX ins Arch User Repository (AUR) hochgeladen wird.

## 🎯 Voraussetzungen

### 1. AUR Account erstellen

1. Gehe zu https://aur.archlinux.org/register
2. Erstelle einen Account mit gültiger Email
3. Bestätige deine Email-Adresse

### 2. SSH Key bei AUR hinterlegen

```bash
# SSH Key generieren (falls noch nicht vorhanden)
ssh-keygen -t ed25519 -C "deine-email@example.com"

# Public Key anzeigen
cat ~/.ssh/id_ed25519.pub
```

1. Gehe zu https://aur.archlinux.org/account/
2. Klicke auf "My Account"
3. Füge deinen Public SSH Key hinzu

### 3. Git konfigurieren

```bash
git config --global user.name "Dein Name"
git config --global user.email "deine-email@example.com"
```

## 📤 Erstmaliger Upload

### Schritt 1: GitHub Release erstellen

```bash
cd /home/inox-network/Projekte/inox-mix

# Git Tag erstellen
git tag -a v0.3.0 -m "Release 0.3.0"
git push origin v0.3.0

# Auf GitHub: Release mit Source-Tarball erstellen
# URL: https://github.com/inox-network/inox-mix/releases/new
```

### Schritt 2: SHA256-Checksum berechnen

```bash
# Tarball herunterladen
wget https://github.com/inox-network/inox-mix/archive/v0.3.0.tar.gz

# Checksum berechnen
sha256sum v0.3.0.tar.gz

# Ergebnis in PKGBUILD eintragen (ersetze 'SKIP')
```

### Schritt 3: AUR Repository erstellen

```bash
# Initial-Repository erstellen (nur beim ersten Mal)
ssh aur@aur.archlinux.org setup-repo inox-mix
```

### Schritt 4: Repository klonen und Dateien hinzufügen

```bash
# AUR Repository klonen
git clone ssh://aur@aur.archlinux.org/inox-mix.git aur-upload
cd aur-upload

# Dateien kopieren
cp ../PKGBUILD .
cp ../inox-mix.install .
cp ../.SRCINFO .

# Commit
git add PKGBUILD inox-mix.install .SRCINFO
git commit -m "Initial release: inox-mix 0.3.0

Professional Audio Mixer for Linux Streamers

Features:
- PipeWire integration
- Real-time VU metering
- Sidechain ducking
- Profanity bleeper
- 8-stage FX chain
- Soundboard support
"

# Push zum AUR
git push origin master
```

### Schritt 5: Verifizieren

1. Gehe zu https://aur.archlinux.org/packages/inox-mix
2. Prüfe ob das Paket sichtbar ist
3. Teste Installation: `yay -S inox-mix`

## 🔄 Update auf neue Version

### Schritt 1: Neue Version vorbereiten

```bash
cd /home/inox-network/Projekte/inox-mix/aur

# PKGBUILD anpassen
vim PKGBUILD
# - pkgver auf neue Version setzen (z.B. 0.3.1)
# - pkgrel auf 1 zurücksetzen
# - sha256sum aktualisieren
```

### Schritt 2: SHA256 aktualisieren

```bash
# Neuen Tarball herunterladen
wget https://github.com/inox-network/inox-mix/archive/v0.3.1.tar.gz

# Neue Checksum
sha256sum v0.3.1.tar.gz

# In PKGBUILD eintragen
```

### Schritt 3: .SRCINFO regenerieren

```bash
makepkg --printsrcinfo > .SRCINFO
```

### Schritt 4: Testen

```bash
# Test-Build
makepkg -f

# Paket validieren
namcap inox-mix-*.pkg.tar.zst

# Test-Installation
sudo pacman -U inox-mix-*.pkg.tar.zst
```

### Schritt 5: Zum AUR pushen

```bash
cd aur-upload

# Neue Dateien kopieren
cp ../PKGBUILD .
cp ../.SRCINFO .

# Commit
git add PKGBUILD .SRCINFO
git commit -m "Update to 0.3.1

Changes:
- Feature X hinzugefügt
- Bug Y gefixt
"

# Push
git push origin master
```

## ✅ Pre-Upload Checklist

- [ ] GitHub Release erstellt mit Source-Tarball
- [ ] SHA256-Checksum in PKGBUILD aktualisiert
- [ ] .SRCINFO generiert (`makepkg --printsrcinfo > .SRCINFO`)
- [ ] PKGBUILD validiert (`namcap PKGBUILD`)
- [ ] Lokaler Test-Build erfolgreich (`makepkg -f`)
- [ ] Paket validiert (`namcap *.pkg.tar.zst`)
- [ ] Test-Installation funktioniert
- [ ] App startet und funktioniert
- [ ] PipeWire-Zugriff funktioniert
- [ ] Desktop-Integration (Icon, Starter) funktioniert
- [ ] Lizenz-Datei vorhanden

## 🔧 Troubleshooting

### "Permission denied (publickey)"

```bash
# SSH-Verbindung testen
ssh aur@aur.archlinux.org

# Sollte antworten: "Hi username, You've successfully authenticated..."
```

### "Package already exists"

Das Paket existiert bereits im AUR. Entweder:
1. Du bist nicht der Owner → Kontaktiere den aktuellen Maintainer
2. Du hast es bereits hochgeladen → Nutze `git pull` statt `git clone`

### "Error: One or more files did not pass the validity check"

```bash
# .SRCINFO neu generieren
makepkg --printsrcinfo > .SRCINFO

# Nochmal committen
git add .SRCINFO
git commit --amend
git push -f
```

## 📞 Support

- AUR Guidelines: https://wiki.archlinux.org/title/AUR_submission_guidelines
- AUR Mailing List: https://lists.archlinux.org/listinfo/aur-general
- IRC: #archlinux-aur auf Libera.Chat
