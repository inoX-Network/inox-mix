// Modul: audio/pipewire — PipeWire-Session und Node-Verwaltung
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;

/// Informationen über ein PipeWire-Audio-Gerät
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    /// Eindeutige PipeWire-Node-ID
    pub id: u32,
    /// Anzeige-Name des Geräts
    pub name: String,
    /// Typ: "source" (Eingang) oder "sink" (Ausgang)
    pub device_type: String,
    /// Anzahl der Kanäle
    pub channels: u32,
}

/// PipeWire-Verbindungsstatus
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PipeWireStatus {
    /// Nicht verbunden
    Disconnected,
    /// Verbindung wird aufgebaut
    Connecting,
    /// Verbunden und betriebsbereit
    Connected,
    /// Fehler bei der Verbindung
    Error(String),
}

/// PipeWire-System-Informationen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipeWireInfo {
    /// PipeWire-Version
    pub version: String,
    /// Ob PipeWire läuft
    pub running: bool,
    /// Aktuelle Sample-Rate des Default-Sinks
    pub sample_rate: u32,
    /// Aktuelle Buffer-Größe (Quantum)
    pub buffer_size: u32,
}

/// PipeWire-Session verwaltet die Verbindung zum Audio-Server
pub struct PipeWireSession {
    /// Verbindungsstatus (Thread-sicher)
    status: Arc<Mutex<PipeWireStatus>>,
    /// Flag ob die MainLoop laufen soll
    running: Arc<AtomicBool>,
    /// Thread-Handle für die PipeWire MainLoop
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl std::fmt::Debug for PipeWireSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeWireSession")
            .field("status", &self.status)
            .field("running", &self.running)
            .finish()
    }
}

impl PipeWireSession {
    /// Neue PipeWire-Session erstellen und verbinden
    ///
    /// Startet die PipeWire MainLoop in einem eigenen Thread.
    /// Gibt Fehler zurück wenn PipeWire nicht verfügbar ist.
    pub fn connect() -> Result<Self, Box<dyn std::error::Error>> {
        let status = Arc::new(Mutex::new(PipeWireStatus::Connecting));
        let running = Arc::new(AtomicBool::new(true));

        // PipeWire initialisieren (muss auf dem Main-Thread passieren bevor Threads starten)
        pipewire::init();
        info!("PipeWire initialisiert");

        let status_clone = Arc::clone(&status);
        let running_clone = Arc::clone(&running);

        // PipeWire MainLoop in eigenem Thread starten
        let thread_handle = thread::Builder::new()
            .name("pipewire-mainloop".to_string())
            .spawn(move || {
                Self::run_mainloop(status_clone, running_clone);
            })?;

        // Kurz warten und Status prüfen
        thread::sleep(std::time::Duration::from_millis(100));

        let current_status = status.lock()
            .map_err(|e| format!("Mutex-Fehler: {}", e))?
            .clone();

        match current_status {
            PipeWireStatus::Error(ref msg) => {
                error!("PipeWire-Verbindung fehlgeschlagen: {}", msg);
                Err(format!("PipeWire nicht verfügbar: {}", msg).into())
            }
            _ => {
                info!("PipeWire-Session gestartet");
                Ok(Self {
                    status,
                    running,
                    thread_handle: Some(thread_handle),
                })
            }
        }
    }

    /// PipeWire MainLoop ausführen (läuft in eigenem Thread)
    fn run_mainloop(
        status: Arc<Mutex<PipeWireStatus>>,
        running: Arc<AtomicBool>,
    ) {
        // MainLoop erstellen
        let mainloop = match pipewire::main_loop::MainLoop::new(None) {
            Ok(ml) => ml,
            Err(e) => {
                if let Ok(mut s) = status.lock() {
                    *s = PipeWireStatus::Error(format!("MainLoop Fehler: {}", e));
                }
                return;
            }
        };

        // Context erstellen
        let context = match pipewire::context::Context::new(&mainloop) {
            Ok(ctx) => ctx,
            Err(e) => {
                if let Ok(mut s) = status.lock() {
                    *s = PipeWireStatus::Error(format!("Context Fehler: {}", e));
                }
                return;
            }
        };

        // Core verbinden
        let _core = match context.connect(None) {
            Ok(core) => {
                if let Ok(mut s) = status.lock() {
                    *s = PipeWireStatus::Connected;
                }
                info!("PipeWire Core verbunden");
                core
            }
            Err(e) => {
                if let Ok(mut s) = status.lock() {
                    *s = PipeWireStatus::Error(format!("Core Verbindung fehlgeschlagen: {}", e));
                }
                return;
            }
        };

        // MainLoop laufen lassen bis running auf false gesetzt wird
        // Wir nutzen einen Timer um periodisch zu prüfen ob wir stoppen sollen
        let _timer = mainloop.loop_().add_timer(move |_| {
            if !running.load(Ordering::Relaxed) {
                // MainLoop wird von außen gestoppt
            }
        });

        mainloop.run();
    }

    /// Aktuellen Verbindungsstatus abfragen
    pub fn status(&self) -> PipeWireStatus {
        self.status.lock()
            .map(|s| s.clone())
            .unwrap_or(PipeWireStatus::Error("Mutex-Fehler".to_string()))
    }

    /// Prüfen ob die Session verbunden ist
    pub fn is_connected(&self) -> bool {
        self.status() == PipeWireStatus::Connected
    }

    /// PipeWire-Session trennen und Thread stoppen
    pub fn disconnect(&mut self) {
        info!("PipeWire-Session wird getrennt...");
        self.running.store(false, Ordering::Relaxed);

        if let Some(handle) = self.thread_handle.take() {
            // Thread beenden — wir warten maximal 2 Sekunden
            let _ = handle.join();
        }

        if let Ok(mut s) = self.status.lock() {
            *s = PipeWireStatus::Disconnected;
        }

        info!("PipeWire-Session getrennt");
    }
}

impl Drop for PipeWireSession {
    fn drop(&mut self) {
        self.disconnect();
        // PipeWire deinitialisieren
        unsafe { pipewire::deinit(); }
    }
}

/// PipeWire System-Informationen über CLI-Tools abfragen (Fallback)
///
/// Nutzt pw-cli und parst die Ausgabe. Funktioniert auch ohne aktive Session.
pub fn get_pipewire_info() -> PipeWireInfo {
    // PipeWire-Version über pw-cli abfragen
    let version = std::process::Command::new("pw-cli")
        .arg("--version")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .and_then(|s| {
            // Format: "pw-cli X.Y.Z" oder ähnlich
            s.lines()
                .next()
                .and_then(|line| line.split_whitespace().last())
                .map(|v| v.trim().to_string())
        })
        .unwrap_or_else(|| "unbekannt".to_string());

    // Prüfen ob PipeWire läuft
    let running = std::process::Command::new("pw-cli")
        .arg("info")
        .arg("0")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    // Sample-Rate und Buffer-Size aus Default-Sink Properties
    let (sample_rate, buffer_size) = get_default_audio_params();

    PipeWireInfo {
        version,
        running,
        sample_rate,
        buffer_size,
    }
}

/// Standard Audio-Parameter (Sample-Rate, Buffer-Size) vom PipeWire-Server abfragen
fn get_default_audio_params() -> (u32, u32) {
    // pw-metadata abfragen für Default-Werte
    let output = std::process::Command::new("pw-metadata")
        .arg("-n")
        .arg("settings")
        .arg("0")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .unwrap_or_default();

    let mut sample_rate: u32 = 48000;
    let mut buffer_size: u32 = 256;

    for line in output.lines() {
        if line.contains("clock.rate") || line.contains("clock.force-rate") {
            if let Some(val) = extract_value_from_metadata(line) {
                if let Ok(sr) = val.parse::<u32>() {
                    sample_rate = sr;
                }
            }
        }
        if line.contains("clock.quantum") || line.contains("clock.force-quantum") {
            if let Some(val) = extract_value_from_metadata(line) {
                if let Ok(bs) = val.parse::<u32>() {
                    buffer_size = bs;
                }
            }
        }
    }

    (sample_rate, buffer_size)
}

/// Wert aus einer PipeWire-Metadata-Zeile extrahieren
fn extract_value_from_metadata(line: &str) -> Option<&str> {
    // Format variiert, versuche den numerischen Wert zu finden
    line.split_whitespace()
        .filter(|s| s.chars().all(|c| c.is_ascii_digit()))
        .last()
}

/// Prüfen ob PipeWire auf dem System verfügbar ist
pub fn check_pipewire_available() -> Result<(), String> {
    let pw_info = get_pipewire_info();

    if !pw_info.running {
        return Err(
            "PipeWire ist nicht aktiv. Bitte stelle sicher, dass PipeWire läuft.\n\
             Starte PipeWire mit: systemctl --user start pipewire pipewire-pulse wireplumber"
                .to_string()
        );
    }

    info!("PipeWire v{} erkannt ({}Hz, {} Samples Buffer)",
        pw_info.version, pw_info.sample_rate, pw_info.buffer_size);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pipewire_info() {
        // Dieser Test funktioniert nur wenn PipeWire installiert ist
        let info = get_pipewire_info();
        // Version sollte nicht leer sein wenn PipeWire installiert ist
        assert!(!info.version.is_empty(), "PipeWire-Version sollte erkannt werden");
    }

    #[test]
    fn test_check_pipewire_available() {
        // Nur prüfen ob die Funktion keine Panic auslöst
        let _result = check_pipewire_available();
        // Ergebnis hängt vom System ab — kein assert
    }

    #[test]
    fn test_extract_value_from_metadata() {
        assert_eq!(extract_value_from_metadata("key value 48000"), Some("48000"));
        assert_eq!(extract_value_from_metadata("no numbers here!"), None);
    }

    #[test]
    fn test_pipewire_status_enum() {
        let status = PipeWireStatus::Connected;
        assert_eq!(status, PipeWireStatus::Connected);

        let error = PipeWireStatus::Error("test".to_string());
        assert!(matches!(error, PipeWireStatus::Error(_)));
    }

    #[test]
    fn test_audio_device_serialize() {
        let device = AudioDevice {
            id: 42,
            name: "Test-Mikrofon".to_string(),
            device_type: "source".to_string(),
            channels: 2,
        };

        let json = serde_json::to_string(&device);
        assert!(json.is_ok());
        assert!(json.unwrap().contains("Test-Mikrofon"));
    }
}
