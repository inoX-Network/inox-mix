# Modul 06: Routing-Matrix

## Zweck
Visuelle Kreuzmatrix: Welcher Input/App geht auf welchen Bus.

## Layout
- Zeilen: Input-Strips + Apps (vertikal)
- Spalten: Output-Busse A1, A2, B1, B2 (horizontal)
- Kreuzungspunkt: Klickbare Zelle (Checkbox-Style)
- Aktiv: Farbig (Bus-Farbe) + Häkchen
- Inaktiv: Leer, dunkler Hintergrund

## Funktionen
- Klick auf Zelle = Verbindung an/aus
- Echtzeit: PipeWire Link erstellen/entfernen
- Farbige Indikatoren pro Bus
- Scrollbar bei vielen Apps

## Tauri Commands
- get_routing_matrix() → Vec<RoutingEntry>
- set_routing(source_id, bus_id, active)

## Tests
- Routing setzen → PipeWire Link vorhanden
- Routing entfernen → PipeWire Link entfernt
