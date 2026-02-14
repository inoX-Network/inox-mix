# Modul 25: Kompatibilität

## Voraussetzung
- PipeWire >= 0.3.30 (95%+ Desktop-Linux)
- wireplumber als Session-Manager

## Bestätigte Distros
| Familie | Distros |
|---------|---------|
| Arch | CachyOS, Arch, Manjaro, EndeavourOS, Garuda |
| Fedora | Fedora 34+, Nobara |
| Ubuntu | 22.10+, Pop!_OS, Mint 21.3+, Zorin |
| Andere | openSUSE, Void, NixOS, Gentoo |

## Audio-Systeme
- PipeWire nativ (primär)
- PulseAudio via pipewire-pulse (Kompatibilitäts-Layer)
- JACK via pipewire-jack
- ALSA direkt (Fallback)

## Installer-Check
- PipeWire installiert und aktiv?
- wireplumber aktiv?
- Version >= 0.3.30?
- Falls nicht: Angebot zur Installation mit distro-spezifischem Befehl
