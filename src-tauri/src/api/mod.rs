// Modul: api — REST + WebSocket Server für externe Controller

pub mod routes;
pub mod websocket;

/// API-Server für externe Steuerung (MIDI-Controller, Mobile App, etc.)
#[derive(Debug)]
pub struct ApiServer {
    // TODO: HTTP Server Handle
    // TODO: WebSocket Connections
    // TODO: Port-Konfiguration
}

impl ApiServer {
    /// API-Server starten
    pub fn start(_port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: REST + WS Server auf Port starten
        todo!("ApiServer::start")
    }

    /// Server stoppen
    pub fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Graceful Shutdown
        todo!("ApiServer::stop")
    }
}
