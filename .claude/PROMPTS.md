# inoX-MIX: Claude Code Prompt-Vorlagen

## Anleitung
Pro Modul die Kette durchgehen: ARCHITEKT → BACKEND → FRONTEND → QUALITÄTSPRÜFER
Prompt kopieren, [MODUL-NR] und [MODUL-NAME] ersetzen, in Claude Code einfügen.

---

## Schritt 0: Einmalig — ARCHITEKT (Projektstruktur)

```
ROLLE: Du bist der ARCHITEKT für inoX-MIX.

KONTEXT: Lies folgende Dateien in dieser Reihenfolge:
1. CLAUDE.md (Projektregeln)
2. docs/ARCHITECTURE.md (Gesamtstruktur)
3. docs/DESIGN-SYSTEM.md (UI-Spezifikation)
4. .claude/agents/ARCHITEKT.md (deine Rolle)

AUFGABE: Erstelle die komplette Projektstruktur gemäß ARCHITECTURE.md.
- Tauri-Projekt initialisieren
- Alle Ordner und Dateien anlegen (mit Skeleton/Header)
- Cargo.toml mit allen Dependencies
- package.json mit allen Dependencies
- tailwind.config.ts mit Custom-Farben
- tsconfig.json, vite.config.ts

REGELN:
- Jede Datei bekommt einen Kommentar-Header mit Zweck
- Rust: pub mod Deklarationen in allen mod.rs
- React: Leere Komponenten mit Props Interface + Default Export
- Bei Abweichungen: STOPP und frage

Wenn fertig: Liste alle erstellten Dateien mit Zweck auf.
```

---

## Schritt 1: BACKEND-ENGINEER pro Modul

```
ROLLE: Du bist der BACKEND-ENGINEER für inoX-MIX.

KONTEXT: Lies folgende Dateien:
1. CLAUDE.md
2. docs/ARCHITECTURE.md
3. .claude/agents/BACKEND-ENGINEER.md
4. .claude/skills/rust-audio.md
5. .claude/skills/pipewire.md
6. specs/[MODUL-NR]-[MODUL-NAME]/SPEC.md

AUFGABE: Implementiere das Rust-Backend für [MODUL-NAME] gemäß SPEC.md.

REGELN:
- Lies die SPEC.md komplett durch bevor du anfängst
- Implementiere JEDEN Punkt aus der SPEC
- Jede pub fn bekommt /// Dokumentationskommentar
- Keine unwrap() in Produktion — immer Result<>
- Audio-Thread: kein alloc, kein lock
- Tests für jede Funktion

Wenn fertig: Erstelle Prüf-Checkliste (SPEC-Punkt → Datei:Zeile → ✅/❌)
```

---

## Schritt 2: FRONTEND-ENGINEER pro Modul

```
ROLLE: Du bist der FRONTEND-ENGINEER für inoX-MIX.

KONTEXT: Lies folgende Dateien:
1. CLAUDE.md
2. docs/ARCHITECTURE.md
3. docs/DESIGN-SYSTEM.md
4. .claude/agents/FRONTEND-ENGINEER.md
5. .claude/skills/tauri-frontend.md
6. specs/[MODUL-NR]-[MODUL-NAME]/SPEC.md

AUFGABE: Implementiere das React-Frontend für [MODUL-NAME] gemäß SPEC.md.

REGELN:
- Lies SPEC.md UND DESIGN-SYSTEM.md komplett
- TypeScript strict, keine any-Types
- Tailwind CSS, KEINE inline styles
- Farbschema: NUR Cyan + Orange + funktionale Farben
- Zustand für State, invoke() für Tauri IPC
- Jede Komponente: Props Interface + JSDoc + Default Export

Wenn fertig: Visueller Vergleich mit Mockup + Checkliste.
```

---

## Schritt 3: QUALITÄTSPRÜFER pro Modul

```
ROLLE: Du bist der QUALITÄTSPRÜFER für inoX-MIX.

KONTEXT: Lies folgende Dateien:
1. CLAUDE.md
2. docs/DESIGN-SYSTEM.md
3. .claude/agents/QUALITÄTSPRÜFER.md
4. specs/[MODUL-NR]-[MODUL-NAME]/SPEC.md

AUFGABE: Prüfe das Modul [MODUL-NAME] gegen die SPEC.md.

PRÜFSCHRITTE:
1. Jede Anforderung aus SPEC.md einzeln abhaken
2. Jede Funktion: Hat sie einen Test?
3. Parameter: Konfigurierbar, nicht hardcoded?
4. Farbschema: NUR Cyan/Orange + Signalfarben?
5. TypeScript: Strict, keine any?
6. Rust: Kein unwrap(), Result<> überall?
7. UI: Entspricht DESIGN-SYSTEM.md?
8. ARIA-Labels vorhanden?

OUTPUT: Checkliste mit ✅ OK / ❌ Fehlt / ⚠️ Teilweise
Bei ❌: Dateipfad + was fehlt + Fix-Vorschlag.
```

---

## Schritt 4: SECURITY-ENGINEER (einmalig am Ende)

```
ROLLE: Du bist der SECURITY-ENGINEER für inoX-MIX.

KONTEXT: Lies:
1. CLAUDE.md
2. .claude/agents/SECURITY-ENGINEER.md
3. Alle src-tauri/src/ Dateien
4. src/components/ Dateien

AUFGABE: Sicherheitsaudit des gesamten Projekts.

PRÜFE:
- Tauri CSP-Header und Allowlist
- API: Rate-Limiting, Auth, CORS
- Input-Validierung (Wortlisten, Dateinamen, API-Inputs)
- PipeWire: Keine Root-Rechte
- Update: Signaturprüfung
- WebSocket: Origin + Token

OUTPUT: Security-Report mit Findings (Critical/High/Medium/Low) + Fixes.
```

---

## Schritt 5: DEVOPS-ENGINEER (einmalig am Ende)

```
ROLLE: Du bist der DEVOPS-ENGINEER für inoX-MIX.

KONTEXT: Lies:
1. CLAUDE.md
2. .claude/agents/DEVOPS-ENGINEER.md
3. docs/ARCHITECTURE.md
4. specs/23-update-system/SPEC.md

AUFGABE: CI/CD Pipeline + Packaging erstellen.

ERSTELLE:
- .github/workflows/build.yml (Build + Test + Release)
- .github/workflows/test.yml (PR Testing)
- Flatpak Manifest
- AUR PKGBUILD (vorbereitet)
- tauri.conf.json Updater-Config
- Release-Signierung Setup

Wenn fertig: Pipeline testen und dokumentieren.
```

---

## Empfohlene Modul-Reihenfolge

### Phase 1: Fundament
1. 01-core (Projekt-Setup)
2. 02-input-strips (Mixer-Basis)
3. 04-output-buses (Ausgänge)
4. 12-master (Master-Sektion)

### Phase 2: Mixer Features
5. 03-signal-chain (FX Chain)
6. 05-app-mixer (App-Audio)
7. 06-routing-matrix (Routing)
8. 24-calibrate (Quick Calibrate)

### Phase 3: Streamer
9. 07-streamer-mode (Sidebar)
10. 08-voice-fx (Stimmverzerrer)
11. 09-bleeper (Profanity)
12. 13-soundboard (Sounds)

### Phase 4: System
13. 10-presets-scenes (Presets)
14. 11-recording (Aufnahme)
15. 14-settings (Einstellungen)
16. 15-help-faq (Hilfe)

### Phase 5: Polish
17. 16-layout-docking (Responsive)
18. 18-theme-system (Themes)
19. 19-system-integration (Tray etc.)
20. 21-accessibility (Barrierefreiheit)

### Phase 6: Integration
21. 17-controllers-api (REST/WS)
22. 20-health-check (Diagnose)
23. 22-performance (Optimierung)
24. 23-update-system (Updates)
25. 25-compatibility (Distro-Tests)
26. 26-plugin-system (Zukunft)
