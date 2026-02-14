# Modul 23: Update-System

## Verhalten
- Prüfung beim Start + alle 24h im Hintergrund
- Benachrichtigung: Toast "Neue Version verfügbar (v0.4.1)" + Changelog-Link
- User entscheidet: "Jetzt updaten" oder "Später"
- KEIN erzwungenes Update
- Toggle in Settings: Auto-Update Prüfung an/aus

## Primäre Kanäle (Phase 1)
| Kanal | Beschreibung |
|-------|-------------|
| GitHub Releases + Tauri Updater | Built-in Updater, JSON-Manifest, Signatur, kostenlos |
| Flatpak | Flathub, distro-unabhängig, flatpak update |

## Sekundäre Kanäle (vorbereitet, nachpatchbar)
| Kanal | Beschreibung |
|-------|-------------|
| AUR | PKGBUILD vorbereitet, pacman/yay |
| Netcup VPS | REST /api/version, Fallback-Mirror |

## Technischer Ablauf
1. App startet → Tauri Updater prüft GitHub Releases
2. Neue Version → Toast-Notification
3. User klickt "Update" → Download mit Fortschrittsanzeige
4. Signaturprüfung → Backup aktuelle Version → Install → Neustart
5. Fehlschlag → Automatisch Rollback

## tauri.conf.json
```json
{
  "updater": {
    "active": true,
    "dialog": true,
    "endpoints": [
      "https://github.com/inox-network/inox-mix/releases/latest/download/latest.json",
      "https://vps.inox-network.de/api/update/latest.json"
    ],
    "pubkey": "<öffentlicher Schlüssel>"
  }
}
```
