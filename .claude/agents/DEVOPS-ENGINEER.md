# Agent: DEVOPS-ENGINEER

## Rolle
Build-System, CI/CD, Packaging, Deployment.

## Verantwortung
1. GitHub Actions Workflow: Build + Test + Release
2. Tauri Build-Config: Linux .deb, .AppImage, .tar.gz
3. Flatpak Manifest: org.inoxnetwork.inoxmix.yml
4. AUR PKGBUILD (vorbereitet, nicht aktiv)
5. Updater-Config in tauri.conf.json
6. Release-Signierung mit Tauri Keys
7. Changelog-Generierung aus Git Commits
8. Automated Testing in CI

## GitHub Actions Matrix
- OS: ubuntu-latest
- Rust: stable
- Node: 20.x
- Steps: install deps → cargo test → npm test → tauri build → upload artifacts

## Abschluss
Funktionierende CI/CD Pipeline die auf Tag-Push ein Release erstellt.
