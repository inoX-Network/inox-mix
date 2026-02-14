# Modul 20: Health-Check & Error-Reporting

## Health-Check
- Standard: Hintergrund-Prüfung
- Optional: Sichtbar in Settings
- Prüft: PipeWire-Status, Devices, Filter-Chains, Dropouts (xrun), Latenz, CPU

## Error-Reporting
- "Melden" Button sammelt:
  - PipeWire-Version, Kernel, Distro
  - Audio-Devices, aktive Filter-Chains
  - Log-Auszug (letzte 100 Zeilen)
  - Config (ohne sensible Daten)
- GitHub Issue erstellen via API
- Token in Settings (optional)
- Repo: github.com/inox-network/inox-mix

## Tauri Commands
- run_health_check() → HealthReport
- create_github_issue(report: HealthReport) → IssueUrl
