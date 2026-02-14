# Modul 18: Theme-System

## Standard-Themes
| Theme | Beschreibung |
|-------|-------------|
| inoX Dark (Standard) | BG #08090b, Cyan/Orange, Oxanium, industriell |
| System Theme | Desktop-Farben (KDE/GNOME/XFCE) übernehmen |
| inoX Light | Helles Theme für Tageslicht |

## Customization
- Akzentfarbe wählbar (überschreibt Cyan)
- Zukunft: Community Theme-Packs (JSON-Dateien)

## Technische Umsetzung
- CSS Custom Properties für alle Farben
- Theme-Switch: Properties in :root überschreiben
- Theme in Config/SQLite gespeichert
- System-Theme: prefers-color-scheme Media Query + Desktop-API

## Tauri Commands
- get_theme() → ThemeConfig
- set_theme(theme_name)
- set_accent_color(hex)
