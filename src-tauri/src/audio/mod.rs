// Modul: audio — Audio-Engine Verwaltung (PipeWire, Mixer, Metering)

pub mod pipewire;
pub mod mixer;
pub mod metering;
pub mod bus;
pub mod routing;

use self::pipewire::{PipeWireSession, PipeWireStatus};
use log::{info, error};

/// Zentrale Audio-Engine die alle Audio-Subsysteme koordiniert
pub struct AudioEngine {
    /// PipeWire-Session (Optional — kann fehlschlagen)
    pw_session: Option<PipeWireSession>,
}

impl std::fmt::Debug for AudioEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioEngine")
            .field("pw_connected", &self.is_connected())
            .finish()
    }
}

impl AudioEngine {
    /// Neue Audio-Engine erstellen und PipeWire verbinden
    ///
    /// Versucht die PipeWire-Verbindung aufzubauen.
    /// Gibt Fehler zurück wenn PipeWire nicht verfügbar ist.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Zuerst prüfen ob PipeWire läuft
        pipewire::check_pipewire_available()
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

        // PipeWire-Session starten
        let pw_session = match PipeWireSession::connect() {
            Ok(session) => {
                info!("Audio-Engine: PipeWire-Session verbunden");
                Some(session)
            }
            Err(e) => {
                error!("Audio-Engine: PipeWire-Verbindung fehlgeschlagen: {}", e);
                return Err(e);
            }
        };

        Ok(Self { pw_session })
    }

    /// Prüfen ob die Audio-Engine mit PipeWire verbunden ist
    pub fn is_connected(&self) -> bool {
        self.pw_session
            .as_ref()
            .map(|s| s.is_connected())
            .unwrap_or(false)
    }

    /// PipeWire-Verbindungsstatus abfragen
    pub fn status(&self) -> PipeWireStatus {
        self.pw_session
            .as_ref()
            .map(|s| s.status())
            .unwrap_or(PipeWireStatus::Disconnected)
    }

    /// Audio-Engine herunterfahren
    pub fn shutdown(&mut self) {
        if let Some(ref mut session) = self.pw_session {
            session.disconnect();
        }
        self.pw_session = None;
        info!("Audio-Engine heruntergefahren");
    }
}
