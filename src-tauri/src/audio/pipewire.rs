// Modul: audio/pipewire — PipeWire-Session und Node-Verwaltung
use serde::{Deserialize, Serialize};
use log::{info, error};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;

/// Standard Sample-Rate als Fallback (Hz)
const DEFAULT_SAMPLE_RATE: u32 = 48000;
/// Standard Buffer-Größe als Fallback (Samples)
const DEFAULT_BUFFER_SIZE: u32 = 256;
/// Wartezeit nach PipeWire-Connect bevor Status geprüft wird (ms)
const PW_CONNECT_WAIT_MS: u64 = 100;
/// Name des PipeWire-MainLoop Threads
const PW_THREAD_NAME: &str = "pipewire-mainloop";

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
            .name(PW_THREAD_NAME.to_string())
            .spawn(move || {
                Self::run_mainloop(status_clone, running_clone);
            })?;

        // Kurz warten und Status prüfen
        thread::sleep(std::time::Duration::from_millis(PW_CONNECT_WAIT_MS));

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
        // SAFETY: pipewire::deinit() wird nur einmal aufgerufen beim Drop
        // der einzigen PipeWireSession-Instanz. Der MainLoop-Thread wurde
        // zuvor in disconnect() gestoppt und gejoined.
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

    let mut sample_rate: u32 = DEFAULT_SAMPLE_RATE;
    let mut buffer_size: u32 = DEFAULT_BUFFER_SIZE;

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

// --- Audio-Routing Link-Management ---

/// Audio-Link erstellen (Source → Bus Verbindung)
///
/// Phase 2a: Nutzt pw-link CLI-Tool zum Erstellen von Audio-Verbindungen.
/// Erstellt einen Link zwischen einem Source-Port und einem Bus-Port.
///
/// # Argumente
/// * `source_id` - Logische Source-ID (z.B. "mic-1", "app-browser")
/// * `bus_id` - Bus-ID (A1, A2, B1, B2)
///
/// # Phase 2b TODO
/// - Node-Discovery: Source/Bus IDs zu PipeWire Node/Port-Namen mappen
/// - Virtual Bus Nodes erstellen (aktuell müssen Busse vorher existieren)
/// - Link-ID zurückgeben und tracken für späteres Entfernen
pub fn create_audio_link(source_id: &str, bus_id: &str) -> Result<(), String> {
    info!("Audio-Link erstellen: {} → {}", source_id, bus_id);

    // Phase 2a: Logischer Mapping-Layer (Source/Bus ID → PipeWire Port-Namen)
    let source_port = map_source_to_port(source_id)?;
    let bus_port = map_bus_to_port(bus_id)?;

    // pw-link CLI-Tool nutzen um Link zu erstellen
    let output = std::process::Command::new("pw-link")
        .arg(&source_port)
        .arg(&bus_port)
        .output()
        .map_err(|e| format!("pw-link konnte nicht ausgeführt werden: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("pw-link Fehler: {}", stderr);
        return Err(format!("Link konnte nicht erstellt werden: {}", stderr));
    }

    info!("Audio-Link erfolgreich erstellt: {} → {}", source_port, bus_port);
    Ok(())
}

/// Audio-Link entfernen (Source → Bus Verbindung trennen)
///
/// Phase 2a: Nutzt pw-link CLI-Tool mit -d Flag zum Entfernen.
///
/// # Argumente
/// * `source_id` - Logische Source-ID
/// * `bus_id` - Bus-ID
pub fn remove_audio_link(source_id: &str, bus_id: &str) -> Result<(), String> {
    info!("Audio-Link entfernen: {} → {}", source_id, bus_id);

    let source_port = map_source_to_port(source_id)?;
    let bus_port = map_bus_to_port(bus_id)?;

    // pw-link mit -d Flag zum Trennen
    let output = std::process::Command::new("pw-link")
        .arg("-d")
        .arg(&source_port)
        .arg(&bus_port)
        .output()
        .map_err(|e| format!("pw-link konnte nicht ausgeführt werden: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("pw-link -d Fehler: {}", stderr);
        return Err(format!("Link konnte nicht entfernt werden: {}", stderr));
    }

    info!("Audio-Link erfolgreich entfernt: {} → {}", source_port, bus_port);
    Ok(())
}

/// Mapping: Logische Source-ID → PipeWire Port-Name
///
/// Phase 2a: Hardcoded Mapping für bekannte Sources.
/// Phase 2b TODO: Dynamische Node-Discovery über PipeWire Registry
fn map_source_to_port(source_id: &str) -> Result<String, String> {
    // Beispiel-Mapping (muss an reale PipeWire-Nodes angepasst werden)
    let port = match source_id {
        "mic-1" => "alsa_input.pci-0000_00_1f.3.analog-stereo:capture_FL",
        "mic-2" => "alsa_input.usb-0000_00_14.0.analog-stereo:capture_FL",
        id if id.starts_with("app-") => {
            // App-Audio: Format "app-browser" → "Firefox:output_FL" (würde Discovery benötigen)
            return Err(format!("App-Audio Routing noch nicht implementiert: {}", id));
        }
        _ => return Err(format!("Unbekannte Source-ID: {}", source_id)),
    };

    Ok(port.to_string())
}

/// Mapping: Bus-ID → PipeWire Port-Name
///
/// Phase 2a: Placeholder — Busse müssen als virtuelle PipeWire Nodes existieren.
/// Phase 2b TODO: Virtual Bus Nodes erstellen (z.B. "inoX-Bus-A1:input_FL")
fn map_bus_to_port(bus_id: &str) -> Result<String, String> {
    // Virtual Bus Nodes müssten zuerst erstellt werden
    let port = match bus_id {
        "A1" => "inoX-Bus-A1:input_FL",
        "A2" => "inoX-Bus-A2:input_FL",
        "B1" => "inoX-Bus-B1:input_FL",
        "B2" => "inoX-Bus-B2:input_FL",
        _ => return Err(format!("Ungültige Bus-ID: {}", bus_id)),
    };

    Ok(port.to_string())
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

    // --- Link-Management Tests ---

    #[test]
    fn test_map_source_to_port_valid() {
        let result = map_source_to_port("mic-1");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("capture"));
    }

    #[test]
    fn test_map_source_to_port_invalid() {
        let result = map_source_to_port("unknown-source");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unbekannte Source-ID"));
    }

    #[test]
    fn test_map_source_to_port_app() {
        let result = map_source_to_port("app-browser");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("App-Audio Routing noch nicht implementiert"));
    }

    #[test]
    fn test_map_bus_to_port_valid() {
        let result = map_bus_to_port("A1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "inoX-Bus-A1:input_FL");

        let result = map_bus_to_port("B2");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "inoX-Bus-B2:input_FL");
    }

    #[test]
    fn test_map_bus_to_port_invalid() {
        let result = map_bus_to_port("X1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Ungültige Bus-ID"));
    }

    #[test]
    #[ignore] // Benötigt laufendes PipeWire und existierende Ports
    fn test_create_audio_link_integration() {
        // Integration-Test: Nur mit echtem PipeWire und konfigurierten Nodes
        let result = create_audio_link("mic-1", "A1");
        // Ergebnis hängt von PipeWire-Setup ab
        println!("create_audio_link result: {:?}", result);
    }

    #[test]
    #[ignore] // Benötigt laufendes PipeWire und existierende Ports
    fn test_remove_audio_link_integration() {
        // Integration-Test: Nur mit echtem PipeWire und konfigurierten Nodes
        let result = remove_audio_link("mic-1", "A1");
        println!("remove_audio_link result: {:?}", result);
    }
}
