// Modul: audio/metering_service — Echtzeit-Metering Service mit Tauri Events
use super::cpal_capture::{AudioSample, CpalCaptureManager};
use super::metering::MeteringEngine;
use super::pipewire;
use log::{error, info, warn};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// Metering-Update-Intervall (16ms ≈ 60fps)
const METERING_INTERVAL_MS: u64 = 16;

/// Metering-Service verwaltet Echtzeit-Metering und sendet Updates an Frontend
pub struct MeteringService {
    /// Metering-Engine (Thread-sicher)
    engine: Arc<Mutex<MeteringEngine>>,
    /// Audio-Buffer für jeden Strip (von CPAL gefüllt)
    audio_buffers: Arc<Mutex<std::collections::HashMap<String, Arc<Mutex<VecDeque<AudioSample>>>>>>,
    /// Flag ob Service läuft
    running: Arc<AtomicBool>,
    /// Thread-Handle für Metering-Loop
    thread_handle: Option<thread::JoinHandle<()>>,
    /// Thread-Handle für CPAL-Manager (separate Thread wegen !Send)
    cpal_thread_handle: Option<thread::JoinHandle<()>>,
    /// Use Real Audio (statt Simulation)
    use_real_audio: Arc<AtomicBool>,
}

impl MeteringService {
    /// Neuen Metering-Service erstellen und starten
    pub fn start(app_handle: AppHandle) -> Self {
        let engine = Arc::new(Mutex::new(MeteringEngine::new()));
        let running = Arc::new(AtomicBool::new(true));
        let use_real_audio = Arc::new(AtomicBool::new(true)); // Versuche CPAL, falle zurück auf Simulation
        let audio_buffers = Arc::new(Mutex::new(std::collections::HashMap::new()));

        let engine_clone = Arc::clone(&engine);
        let running_clone = Arc::clone(&running);
        let use_real_audio_clone = Arc::clone(&use_real_audio);
        let audio_buffers_clone = Arc::clone(&audio_buffers);

        // ═══════════════════════════════════════════════════════════════
        // Phase 2c: Dynamische Strip-Registrierung via PipeWire Discovery
        // ═══════════════════════════════════════════════════════════════

        info!("🔍 Starte PipeWire Node-Discovery...");

        match pipewire::list_audio_devices() {
            Ok(devices) => {
                let input_count = devices.iter().filter(|d| d.device_type == "input").count();
                let output_count = devices.iter().filter(|d| d.device_type == "output").count();

                info!(
                    "✅ PipeWire-Nodes gefunden: {} Input, {} Output",
                    input_count, output_count
                );

                if let Ok(mut eng) = engine.lock() {
                    let mut hw_mic_counter = 1;
                    let mut virt_counter = 1;

                    for device in devices.iter().filter(|d| d.device_type == "input") {
                        // Intelligentes Strip-ID Mapping basierend auf Node-Properties
                        let strip_id = if device.name.contains("alsa_input") {
                            // Hardware-Input (ALSA)
                            let id = if device.name.contains("analog") {
                                format!("hw-mic-{}", hw_mic_counter)
                            } else if device.name.contains("usb") {
                                hw_mic_counter += 1;
                                format!("hw-mic-{}", hw_mic_counter)
                            } else {
                                format!("hw-input-{}", device.id)
                            };
                            hw_mic_counter += 1;
                            id
                        } else if device.name.contains("application")
                            || device.name.contains("client")
                        {
                            // Virtual-Input (Application-Stream)
                            let app_name = extract_app_name(&device.name);
                            format!("virt-{}", app_name)
                        } else if device.name.contains("loopback")
                            || device.name.contains("monitor")
                        {
                            // Loopback/Monitor-Device
                            format!("virt-loop-{}", virt_counter)
                        } else {
                            // Generic Input
                            let id = format!("virt-{}", virt_counter);
                            virt_counter += 1;
                            id
                        };

                        eng.register_strip(&strip_id);
                        info!(
                            "  ✓ Strip registriert: {} ← {} (Node {}, {}ch)",
                            strip_id,
                            truncate_name(&device.name, 40),
                            device.id,
                            device.channels
                        );
                    }

                    let total_strips = eng.get_levels().len();
                    info!("📊 Metering: {} Strips aktiv", total_strips);
                }
            }
            Err(e) => {
                warn!("⚠️  PipeWire Node-Discovery fehlgeschlagen: {}", e);
                warn!("📦 Nutze Fallback-Strips (Demo-Modus)");

                // Fallback: Standard-Strips für Demo
                if let Ok(mut eng) = engine.lock() {
                    eng.register_strip("hw-mic-1");
                    eng.register_strip("hw-mic-2");
                    eng.register_strip("virt-browser");
                    eng.register_strip("virt-spotify");
                    info!("  ✓ 4 Fallback-Strips registriert");
                }
            }
        }

        // CPAL Audio-Capture in separatem Thread starten (da Streams nicht Send sind)
        let cpal_thread_handle = if use_real_audio.load(Ordering::Relaxed) {
            let audio_buffers_cpal = Arc::clone(&audio_buffers);
            let use_real_audio_cpal = Arc::clone(&use_real_audio);
            let running_cpal = Arc::clone(&running);
            let engine_cpal = Arc::clone(&engine);

            thread::Builder::new()
                .name("cpal-capture".to_string())
                .spawn(move || {
                    info!("🎤 CPAL Capture-Thread gestartet");

                    // CPAL Manager in diesem Thread erstellen (Streams bleiben hier!)
                    let mut cpal_manager = match CpalCaptureManager::new() {
                        Ok(manager) => {
                            info!("✅ CPAL initialisiert in Capture-Thread");
                            manager
                        }
                        Err(e) => {
                            warn!("⚠️  CPAL-Init fehlgeschlagen: {} - Nutze Simulation", e);
                            use_real_audio_cpal.store(false, Ordering::Relaxed);
                            return;
                        }
                    };

                    // Default Input-Device starten
                    if let Ok(device) = cpal_manager.get_default_input_device() {
                        match cpal_manager.start_capture(device, "default-input") {
                            Ok(buffer) => {
                                audio_buffers_cpal
                                    .lock()
                                    .unwrap()
                                    .insert("default-input".to_string(), buffer);
                                info!("🎤 Default Input-Device Capture gestartet");

                                // Strip für Default-Input registrieren
                                if let Ok(mut eng) = engine_cpal.lock() {
                                    eng.register_strip("default-input");
                                }
                            }
                            Err(e) => {
                                warn!("⚠️  Default-Input Capture fehlgeschlagen: {}", e);
                                use_real_audio_cpal.store(false, Ordering::Relaxed);
                            }
                        }
                    }

                    // CPAL-Thread läuft weiter solange MeteringService aktiv ist
                    // Streams müssen am Leben bleiben!
                    while running_cpal.load(Ordering::Relaxed) {
                        thread::sleep(Duration::from_millis(100));
                    }

                    info!("🎤 CPAL Capture-Thread beendet");
                    cpal_manager.shutdown();
                })
                .ok()
        } else {
            None
        };

        // Metering-Thread starten
        let thread_handle = thread::Builder::new()
            .name("metering-service".to_string())
            .spawn(move || {
                Self::run_metering_loop(
                    engine_clone,
                    running_clone,
                    use_real_audio_clone,
                    audio_buffers_clone,
                    app_handle,
                );
            })
            .ok();

        info!("✅ Metering-Service gestartet");

        Self {
            engine,
            audio_buffers,
            running,
            thread_handle,
            cpal_thread_handle,
            use_real_audio,
        }
    }

    /// Metering-Loop (läuft in eigenem Thread)
    fn run_metering_loop(
        engine: Arc<Mutex<MeteringEngine>>,
        running: Arc<AtomicBool>,
        use_real_audio: Arc<AtomicBool>,
        audio_buffers: Arc<
            Mutex<std::collections::HashMap<String, Arc<Mutex<VecDeque<AudioSample>>>>>,
        >,
        app_handle: AppHandle,
    ) {
        while running.load(Ordering::Relaxed) {
            // Metering-Daten sammeln
            let levels = if let Ok(eng) = engine.lock() {
                eng.get_levels()
            } else {
                error!("Metering-Engine Lock-Fehler");
                Vec::new()
            };

            // ═══════════════════════════════════════════════════════════════
            // Phase 2d: ECHTES AUDIO via CPAL oder Fallback Simulation
            // ═══════════════════════════════════════════════════════════════

            if use_real_audio.load(Ordering::Relaxed) {
                // ✅ ECHTES AUDIO von CPAL
                if let Ok(mut eng) = engine.lock() {
                    let buffers = audio_buffers.lock().unwrap();

                    for (strip_id, buffer_arc) in buffers.iter() {
                        let mut buffer = buffer_arc.lock().unwrap();

                        // Samples aus Buffer lesen (max 256 Samples ≈ 5.3ms @ 48kHz)
                        let sample_count = buffer.len().min(256);
                        let mut samples = Vec::with_capacity(sample_count * 2); // Stereo

                        for _ in 0..sample_count {
                            if let Some(sample) = buffer.pop_front() {
                                samples.push(sample.left);
                                samples.push(sample.right);
                            }
                        }

                        if !samples.is_empty() {
                            eng.process_buffer(strip_id, &samples, 2);
                        }
                    }
                }
            } else {
                // ⚠️  SIMULATION (Fallback)
                if let Ok(mut eng) = engine.lock() {
                    let registered_strips: Vec<String> = eng
                        .get_levels()
                        .iter()
                        .map(|l| l.strip_id.clone())
                        .collect();

                    if !registered_strips.is_empty() {
                        simulate_audio_for_strips(
                            &mut eng,
                            &registered_strips
                                .iter()
                                .map(|s| s.as_str())
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }

            // Events an Frontend senden
            if !levels.is_empty() {
                if let Err(e) = app_handle.emit("metering-update", &levels) {
                    error!("Fehler beim Senden der Metering-Daten: {}", e);
                }
            }

            // Warten bis nächstes Update
            thread::sleep(Duration::from_millis(METERING_INTERVAL_MS));
        }

        info!("Metering-Loop beendet");
    }

    /// Strip für Metering registrieren
    pub fn register_strip(&self, strip_id: &str) {
        if let Ok(mut engine) = self.engine.lock() {
            engine.register_strip(strip_id);
            info!("Strip für Metering registriert: {}", strip_id);
        }
    }

    /// Strip aus Metering entfernen
    pub fn unregister_strip(&self, strip_id: &str) {
        if let Ok(mut engine) = self.engine.lock() {
            engine.unregister_strip(strip_id);
            info!("Strip aus Metering entfernt: {}", strip_id);
        }
    }

    /// Clipping für Strip zurücksetzen
    pub fn reset_clipping(&self, strip_id: &str) {
        if let Ok(mut engine) = self.engine.lock() {
            engine.reset_clipping(strip_id);
        }
    }

    /// Metering-Service stoppen
    pub fn stop(&mut self) {
        info!("🛑 Metering-Service wird gestoppt...");
        self.running.store(false, Ordering::Relaxed);

        // Metering-Thread stoppen
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }

        // CPAL Capture-Thread stoppen
        if let Some(handle) = self.cpal_thread_handle.take() {
            let _ = handle.join();
        }

        info!("✅ Metering-Service gestoppt");
    }
}

impl Drop for MeteringService {
    fn drop(&mut self) {
        self.stop();
    }
}

/// App-Namen aus PipeWire-Node-Namen extrahieren
fn extract_app_name(node_name: &str) -> String {
    // Beispiele:
    // "Firefox.instance123" → "firefox"
    // "Chromium-browser" → "chromium"
    // "com.spotify.Client" → "spotify"

    node_name
        .split('.')
        .next()
        .unwrap_or("app")
        .split('-')
        .next()
        .unwrap_or("app")
        .to_lowercase()
}

/// Namen auf maximale Länge kürzen (für Logging)
fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("{}...", &name[..max_len - 3])
    }
}

/// Simuliere Audio-Daten für Test/Demo
///
/// Phase 2c: Generiert sinusförmige Test-Signale für dynamisch registrierte Strips
/// Phase 2d (CPAL): Ersetzt durch echte Audio-Capture
///
/// Aktuell: Simulation läuft nur für tatsächlich existierende PipeWire-Nodes
fn simulate_audio_for_strips(engine: &mut MeteringEngine, strip_ids: &[&str]) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for strip_id in strip_ids {
        // Amplitude variiert je nach Strip-Typ für realistisches Verhalten
        let amplitude: f32 = if strip_id.starts_with("hw-") {
            // Hardware-Mic: Moderater Pegel
            rng.gen_range(0.3..0.6)
        } else if strip_id.starts_with("virt-") {
            // Virtual/App-Audio: Höherer Pegel
            rng.gen_range(0.4..0.8)
        } else {
            // Default
            rng.gen_range(0.1..0.5)
        };

        // Frequenz variiert je nach Strip für unterscheidbare Signale
        let freq_l = if strip_id.contains("mic-1") {
            440.0 // A4
        } else if strip_id.contains("mic-2") {
            554.37 // C#5
        } else {
            rng.gen_range(200.0..800.0)
        };

        let freq_r = freq_l * 1.5; // Harmonisch unterschiedlich

        // Sinusförmiges Signal generieren (256 Samples @ 48kHz ≈ 5.3ms)
        let mut samples = Vec::with_capacity(512);
        for i in 0..256 {
            let t = i as f32 / 48000.0; // Zeit in Sekunden
            let sample_l = amplitude * (2.0 * std::f32::consts::PI * freq_l * t).sin();
            let sample_r = amplitude * 0.85 * (2.0 * std::f32::consts::PI * freq_r * t).sin();
            samples.push(sample_l);
            samples.push(sample_r);
        }

        engine.process_buffer(strip_id, &samples, 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metering_service_creation() {
        // Dieser Test würde einen AppHandle benötigen
        // In echten Tests würde man einen Mock-AppHandle verwenden
    }

    #[test]
    fn test_simulate_audio_for_strips() {
        let mut engine = MeteringEngine::new();
        engine.register_strip("test");

        simulate_audio_for_strips(&mut engine, &["test"]);

        let levels = engine.get_strip_levels("test");
        assert!(levels.is_some());

        let levels = levels.unwrap();
        // Simulierte Daten sollten Pegel > -60dB haben
        assert!(levels.peak_l > -60.0);
        assert!(levels.rms_l > -60.0);
    }
}
