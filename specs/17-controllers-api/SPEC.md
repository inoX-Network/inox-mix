# Modul 17: Externe Controller & API

## Controller-Support
| Controller | Protokoll | Features |
|-----------|-----------|---------|
| Stream Deck | streamdeck-ui Plugin | Mute, Szenen, FX, Soundboard, Recording |
| TouchPortal | TCP/IP | Handy/Tablet als Controller |
| MIDI | MIDI Messages | Behringer X-Touch, Korg nanoKONTROL etc. |
| OSC | Open Sound Control | DAW-Integration |
| Handy-Remote | HTTP/WS | Browser unter 192.168.x.x:8080 |

## REST API
- Port: 8080 (konfigurierbar)
- Auth: Optionaler API-Key
- Endpoints: GET/POST für alle Mixer-Funktionen
- Dokumentiert mit OpenAPI/Swagger

## WebSocket API
- Echtzeit-Updates: Levels, State-Changes
- Bidirektional: Commands empfangen + Status senden
- JSON Messages

## Rust-Backend
- src-tauri/src/api/mod.rs: Server Setup
- src-tauri/src/api/routes.rs: REST Endpoints
- src-tauri/src/api/websocket.rs: WS Handler

## API Endpoints (Beispiele)
- GET /api/strips → Alle Strips + Status
- POST /api/strips/{id}/volume → Volume setzen
- POST /api/strips/{id}/mute → Mute Toggle
- GET /api/scenes → Alle Szenen
- POST /api/scenes/{name}/load → Szene laden
- WS /ws → Echtzeit-Stream
