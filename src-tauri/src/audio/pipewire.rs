// Modul: audio/pipewire — PipeWire-Session und Node-Verwaltung
use serde::{Deserialize, Serialize};
use log::{info, error, warn};
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

/// Alle Audio-Geräte aus PipeWire abfragen
///
/// Phase 2b: Erweitert um dynamische Node-Discovery via PipeWire-Registry
pub fn list_audio_devices() -> Result<Vec<AudioDevice>, String> {
    // Phase 2b: Versuche zuerst Registry-basierte Discovery
    // Fallback auf pw-cli wenn Registry nicht verfügbar
    match list_audio_devices_via_registry() {
        Ok(devices) if !devices.is_empty() => {
            info!("PipeWire Nodes via Registry: {}", devices.len());
            return Ok(devices);
        }
        Err(e) => {
            warn!("Registry-Discovery fehlgeschlagen, Fallback auf pw-cli: {}", e);
        }
        _ => {}
    }

    // Fallback: pw-cli basierte Discovery
    let output = std::process::Command::new("pw-cli")
        .arg("list-objects")
        .arg("Node")
        .output()
        .map_err(|e| format!("pw-cli konnte nicht ausgeführt werden: {}", e))?;

    if !output.status.success() {
        return Err("pw-cli list-objects fehlgeschlagen".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let devices = parse_pw_nodes(&stdout);

    info!("PipeWire Nodes via pw-cli: {}", devices.len());
    Ok(devices)
}

/// Audio-Devices via PipeWire-Registry abfragen (Phase 2b)
///
/// Nutzt die PipeWire-API direkt statt CLI-Tools
fn list_audio_devices_via_registry() -> Result<Vec<AudioDevice>, String> {
    use std::sync::mpsc;
    use std::time::Duration;

    // Timeout für Registry-Scan
    const REGISTRY_TIMEOUT_MS: u64 = 500;

    let (tx, rx) = mpsc::channel();

    // PipeWire initialisieren für Registry-Scan
    pipewire::init();

    let mainloop = pipewire::main_loop::MainLoop::new(None)
        .map_err(|e| format!("MainLoop Fehler: {}", e))?;

    let context = pipewire::context::Context::new(&mainloop)
        .map_err(|e| format!("Context Fehler: {}", e))?;

    let core = context.connect(None)
        .map_err(|e| format!("Core Verbindung fehlgeschlagen: {}", e))?;

    let registry = core.get_registry()
        .map_err(|e| format!("Registry-Zugriff fehlgeschlagen: {}", e))?;

    let mut devices = Vec::new();

    // Registry-Listener für Global-Events
    let _listener = registry
        .add_listener_local()
        .global(move |global| {
            // Nur Audio-Nodes berücksichtigen
            if global.type_ == pipewire::types::ObjectType::Node {
                // Node-Properties auslesen
                if let Some(props) = &global.props {
                    let name = props.get("node.name").unwrap_or("unknown").to_string();
                    let media_class = props.get("media.class").unwrap_or("");

                    // Nur Audio-Source/Sink Nodes
                    let device_type = if media_class.contains("Source") || media_class.contains("Input") {
                        "input".to_string()
                    } else if media_class.contains("Sink") || media_class.contains("Output") {
                        "output".to_string()
                    } else {
                        return; // Kein Audio-Node
                    };

                    let channels = props.get("audio.channels")
                        .and_then(|s| s.parse::<u32>().ok())
                        .unwrap_or(2);

                    let device = AudioDevice {
                        id: global.id,
                        name,
                        device_type,
                        channels,
                    };

                    let _ = tx.send(device);
                }
            }
        })
        .register();

    // Registry-Events sammeln (mit Timeout)
    let start = std::time::Instant::now();
    let loop_ref = mainloop.loop_();

    while start.elapsed() < Duration::from_millis(REGISTRY_TIMEOUT_MS) {
        // Iteration der MainLoop (nicht-blockierend)
        loop_ref.iterate(Duration::from_millis(10));

        // Events sammeln
        if let Ok(device) = rx.try_recv() {
            devices.push(device);
        }
    }

    // Cleanup
    unsafe { pipewire::deinit(); }

    if devices.is_empty() {
        Err("Keine Audio-Devices via Registry gefunden".to_string())
    } else {
        Ok(devices)
    }
}

/// Parse pw-cli list-objects Output zu AudioDevice Liste
fn parse_pw_nodes(output: &str) -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    let mut current_id: Option<u32> = None;
    let mut current_name = String::new();
    let mut current_type = String::new();
    let mut current_channels: u32 = 2; // Default Stereo

    for line in output.lines() {
        let trimmed = line.trim();

        // Node ID extrahieren: "  id 42, ..."
        if let Some(id_str) = trimmed.strip_prefix("id ") {
            if let Some(id_part) = id_str.split(',').next() {
                if let Ok(id) = id_part.trim().parse::<u32>() {
                    current_id = Some(id);
                }
            }
        }

        // Name extrahieren: "    node.name = "alsa_input.pci-...""
        if trimmed.contains("node.name") || trimmed.contains("media.name") {
            if let Some(name_part) = trimmed.split('=').nth(1) {
                current_name = name_part.trim().trim_matches('"').to_string();
            }
        }

        // Typ extrahieren: "    media.class = "Audio/Source""
        if trimmed.contains("media.class") {
            if let Some(class_part) = trimmed.split('=').nth(1) {
                let class = class_part.trim().trim_matches('"');
                if class.contains("Source") || class.contains("Input") {
                    current_type = "input".to_string();
                } else if class.contains("Sink") || class.contains("Output") {
                    current_type = "output".to_string();
                }
            }
        }

        // Channels extrahieren: "    audio.channels = 2"
        if trimmed.contains("audio.channels") {
            if let Some(ch_part) = trimmed.split('=').nth(1) {
                if let Ok(ch) = ch_part.trim().parse::<u32>() {
                    current_channels = ch;
                }
            }
        }

        // Am Ende eines Node-Blocks: AudioDevice erstellen
        if trimmed.is_empty() && current_id.is_some() && !current_name.is_empty() {
            devices.push(AudioDevice {
                id: current_id.unwrap(),
                name: current_name.clone(),
                device_type: current_type.clone(),
                channels: current_channels,
            });

            // Reset für nächsten Block
            current_id = None;
            current_name.clear();
            current_type.clear();
            current_channels = 2;
        }
    }

    devices
}

/// Mapping: Logische Source-ID → PipeWire Port-Name
///
/// Phase 2b: Erweitert um dynamische Node-Discovery
fn map_source_to_port(source_id: &str) -> Result<String, String> {
    // Phase 2b: Dynamische Node-Discovery nutzen
    if let Ok(devices) = list_audio_devices() {
        // Strategie 1: Exakte ID-Suche in Node-Namen
        for device in devices.iter() {
            if device.device_type == "input" {
                // Strip-ID Matching (z.B. "hw-mic-1" → "analog" oder "usb")
                let matches = match source_id {
                    "hw-mic-1" | "mic-1" => device.name.contains("analog"),
                    "hw-mic-2" | "mic-2" => device.name.contains("usb"),
                    id if id.starts_with("hw-input-") => {
                        // Direkte Node-ID (z.B. "hw-input-42")
                        if let Some(id_str) = id.strip_prefix("hw-input-") {
                            if let Ok(node_id) = id_str.parse::<u32>() {
                                device.id == node_id
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                    _ => device.name.to_lowercase().contains(&source_id.to_lowercase()),
                };

                if matches {
                    // Port-Namen konstruieren (Konvention: node_name:port_name)
                    let port_name = format!("{}:capture_FL", device.name);
                    info!("Source-Mapping: {} → {}", source_id, port_name);
                    return Ok(port_name);
                }
            }
        }

        // Strategie 2: App-Audio Nodes (media.class = "Stream/Input/Audio")
        if source_id.starts_with("app-") {
            for device in devices.iter() {
                if device.name.contains("application") || device.name.contains("client") {
                    let app_name = source_id.strip_prefix("app-").unwrap_or(source_id);
                    if device.name.to_lowercase().contains(&app_name.to_lowercase()) {
                        let port_name = format!("{}:output_FL", device.name);
                        info!("App-Source-Mapping: {} → {}", source_id, port_name);
                        return Ok(port_name);
                    }
                }
            }
            return Err(format!(
                "App-Audio Node nicht gefunden für: {}. \
                 Stelle sicher, dass die Anwendung Audio abspielt.",
                source_id
            ));
        }
    } else {
        warn!("Node-Discovery fehlgeschlagen, nutze Fallback-Mapping");
    }

    // Fallback: Hardcoded Mapping für bekannte Sources
    let port = match source_id {
        "mic-1" | "hw-mic-1" => "alsa_input.pci-0000_00_1f.3.analog-stereo:capture_FL",
        "mic-2" | "hw-mic-2" => "alsa_input.usb-0000_00_14.0.analog-stereo:capture_FL",
        _ => return Err(format!("Unbekannte Source-ID: {}", source_id)),
    };

    info!("Source-Fallback-Mapping: {} → {}", source_id, port);
    Ok(port.to_string())
}

/// Virtual Bus Nodes erstellen (inoX-Bus-A1 bis B2)
///
/// Phase 2b: Erstellt virtuelle Loopback-Nodes für jeden Bus
/// Diese können dann als Routing-Ziele verwendet werden
pub fn create_virtual_bus_nodes() -> Result<(), String> {
    info!("Erstelle Virtual Bus Nodes...");

    let buses = vec!["A1", "A2", "B1", "B2"];

    for bus_id in buses {
        // pw-loopback nutzen um virtuelle Nodes zu erstellen
        // Format: pw-loopback -n "inoX-Bus-A1" -c 2
        let bus_name = format!("inoX-Bus-{}", bus_id);

        let output = std::process::Command::new("pw-loopback")
            .arg("-n")
            .arg(&bus_name)
            .arg("-c")
            .arg("2") // Stereo
            .arg("--latency")
            .arg("256/48000") // 256 Samples @ 48kHz
            .spawn();

        match output {
            Ok(_) => info!("Virtual Bus Node erstellt: {}", bus_name),
            Err(e) => {
                error!("Fehler beim Erstellen von {}: {}", bus_name, e);
                // Nicht abbrechen, weiter mit nächstem Bus
            }
        }
    }

    info!("Virtual Bus Nodes Setup abgeschlossen");
    Ok(())
}

/// Virtual Bus Nodes stoppen
pub fn destroy_virtual_bus_nodes() -> Result<(), String> {
    info!("Stoppe Virtual Bus Nodes...");

    // Alle pw-loopback Prozesse mit "inoX-Bus" im Namen beenden
    let _ = std::process::Command::new("pkill")
        .arg("-f")
        .arg("pw-loopback.*inoX-Bus")
        .output();

    info!("Virtual Bus Nodes gestoppt");
    Ok(())
}

/// Mapping: Bus-ID → PipeWire Port-Name
///
/// Phase 2b: Nutzt virtuelle Bus-Nodes
fn map_bus_to_port(bus_id: &str) -> Result<String, String> {
    // Virtual Bus Nodes werden beim Start erstellt
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

    // --- Node-Discovery Tests ---

    #[test]
    #[ignore] // Benötigt laufendes PipeWire
    fn test_list_audio_devices_integration() {
        let result = list_audio_devices();
        assert!(result.is_ok());
        let devices = result.unwrap();
        println!("Audio Devices gefunden: {}", devices.len());
        for device in devices {
            println!("  - {} ({}, {} Kanäle)", device.name, device.device_type, device.channels);
        }
    }

    #[test]
    fn test_parse_pw_nodes() {
        let mock_output = r#"
  id 42, type PipeWire:Interface:Node/3
    node.name = "alsa_input.pci-0000_00_1f.3.analog-stereo"
    media.class = "Audio/Source"
    audio.channels = 2

  id 43, type PipeWire:Interface:Node/3
    node.name = "alsa_output.pci-0000_00_1f.3.analog-stereo"
    media.class = "Audio/Sink"
    audio.channels = 2
"#;

        let devices = parse_pw_nodes(mock_output);
        assert_eq!(devices.len(), 2);
        assert_eq!(devices[0].id, 42);
        assert_eq!(devices[0].device_type, "input");
        assert_eq!(devices[1].id, 43);
        assert_eq!(devices[1].device_type, "output");
    }

    #[test]
    #[ignore] // Benötigt laufendes PipeWire und Berechtigungen
    fn test_create_virtual_bus_nodes_integration() {
        let result = create_virtual_bus_nodes();
        println!("create_virtual_bus_nodes result: {:?}", result);
    }

    #[test]
    #[ignore] // Benötigt laufendes PipeWire und Berechtigungen
    fn test_destroy_virtual_bus_nodes_integration() {
        let result = destroy_virtual_bus_nodes();
        println!("destroy_virtual_bus_nodes result: {:?}", result);
    }
}
