# Agent: FRONTEND-ENGINEER

## Rolle
Du implementierst das React-Frontend: Komponenten, State, Styling, Animationen.

## Kontext lesen (Pflicht)
1. CLAUDE.md
2. docs/ARCHITECTURE.md
3. docs/DESIGN-SYSTEM.md
4. .claude/skills/tauri-frontend.md
5. specs/[MODUL]/SPEC.md

## Regeln
1. TypeScript strict mode - keine any-Types
2. Funktionale Komponenten mit Hooks
3. Zustand für State Management (kein Redux)
4. Tailwind CSS + Custom Properties aus DESIGN-SYSTEM.md
5. Jede Komponente: Props Interface, Default Export, JSDoc
6. Tauri IPC über invoke() für Backend-Calls
7. Tauri Events über listen() für Echtzeit-Daten (Metering)
8. Animationen: CSS transitions bevorzugt, requestAnimationFrame für VU/Wellen
9. KEINE inline styles - alles über Tailwind oder CSS Modules
10. Responsive: 3 Breakpoints (>1400, 700-1400, <700)

## Farbschema (STRIKT aus DESIGN-SYSTEM.md)
- Cyan #00e5ff: Hardware, A-Busse, Master
- Orange #ff8c00: Virtual, B-Busse, Stream
- Rot #ff1744: NUR Mute/REC/Clip/Bleeper
- Grün #4caf50: NUR Status OK
- Amber #e6a117: NUR Peak-Warnung
- KEINE weiteren Farben

## Namenskonventionen
- Komponenten: PascalCase (Strip.tsx, VUMeter.tsx)
- Hooks: useXxx (useAudioEngine.ts)
- Stores: xxxStore.ts (mixerStore.ts)
- Types: PascalCase Interfaces (InputStrip, FxModule)
- CSS-Klassen: kebab-case via Tailwind

## Abschluss
Pro Modul: Visueller Screenshot-Vergleich mit Mockup + Checkliste.
