// Modul: audio/metering_service — Echtzeit-Metering Service mit Tauri Events
use super::metering::{MeteringEngine, StripLevels};
use super::pipewire;
use log::{info, error, warn};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// Metering-Update-Intervall (16ms ≈ 60fps)
const METERING_INTERVAL_MS: u64 = 16;

/// Metering-Service verwaltet Echtzeit-Metering und sendet Updates an Frontend
pub struct MeteringService {
    /// Metering-Engine (Thread-sicher)
    engine: Arc<Mutex<MeteringEngine>>,
    /// Flag ob Service läuft
    running: Arc<AtomicBool>,
    /// Thread-Handle
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl MeteringService {
    /// Neuen Metering-Service erstellen und starten
    pub fn start(app_handle: AppHandle) -> Self {
        let engine = Arc::new(Mutex::new(MeteringEngine::new()));
        let running = Arc::new(AtomicBool::new(true));

        let engine_clone = Arc::clone(&engine);
        let running_clone = Arc::clone(&running);

        // Discover PipeWire Audio-Nodes beim Start
        if let Ok(devices) = pipewire::list_audio_devices() {
            info!("PipeWire Audio-Geräte gefunden: {}", devices.len());
            for device in &devices {
                info!(
                    "  - {} (ID: {}, Typ: {}, Kanäle: {})",
                    device.name, device.id, device.device_type, device.channels
                );
            }

            // Standard-Strips für bekannte Input-Devices registrieren
            if let Ok(mut eng) = engine.lock() {
                for device in devices.iter().filter(|d| d.device_type == "input") {
                    // Strip-ID aus Node-Namen ableiten
                    let strip_id = if device.name.contains("analog") {
                        "hw-mic-1".to_string()
                    } else if device.name.contains("usb") {
                        "hw-mic-2".to_string()
                    } else {
                        format!("hw-input-{}", device.id)
                    };

                    eng.register_strip(&strip_id);
                    info!("Strip registriert: {} → {}", strip_id, device.name);
                }
            }
        } else {
            warn!("PipeWire Node-Discovery fehlgeschlagen, nutze Fallback-Strips");
            // Fallback: Standard-Strips registrieren
            if let Ok(mut eng) = engine.lock() {
                eng.register_strip("hw-mic-1");
                eng.register_strip("hw-mic-2");
            }
        }

        // Metering-Thread starten
        let thread_handle = thread::Builder::new()
            .name("metering-service".to_string())
            .spawn(move || {
                Self::run_metering_loop(engine_clone, running_clone, app_handle);
            })
            .ok();

        info!("Metering-Service gestartet");

        Self {
            engine,
            running,
            thread_handle,
        }
    }

    /// Metering-Loop (läuft in eigenem Thread)
    fn run_metering_loop(
        engine: Arc<Mutex<MeteringEngine>>,
        running: Arc<AtomicBool>,
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

            // Phase 2b: Audio-Daten aus PipeWire-Capture
            // TODO Phase 2c: Ersetze Simulation durch echte Capture
            //
            // Implementierung:
            // 1. AudioCaptureManager.start_capture() für jeden registrierten Strip
            // 2. Audio-Samples aus Ring-Buffer (Consumer) lesen
            // 3. MeteringEngine.process_buffer() mit echten Samples füttern
            //
            // Für jetzt: Simulierte Daten für registrierte Strips
            if let Ok(mut eng) = engine.lock() {
                let registered_strips: Vec<String> = eng.get_levels()
                    .iter()
                    .map(|l| l.strip_id.clone())
                    .collect();

                if !registered_strips.is_empty() {
                    simulate_audio_for_strips(
                        &mut eng,
                        &registered_strips.iter().map(|s| s.as_str()).collect::<Vec<_>>()
                    );
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
        info!("Metering-Service wird gestoppt...");
        self.running.store(false, Ordering::Relaxed);

        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }

        info!("Metering-Service gestoppt");
    }
}

impl Drop for MeteringService {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Simuliere Audio-Daten für Test/Demo
///
/// Phase 1: Generiert sinusförmige Test-Signale
/// Phase 2: Ersetzt durch echte PipeWire-Capture
fn simulate_audio_for_strips(engine: &mut MeteringEngine, strip_ids: &[&str]) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for strip_id in strip_ids {
        // Zufällige Amplitude zwischen 0.1 und 0.7 für realistische VU-Meter
        let amplitude: f32 = rng.gen_range(0.1..0.7);

        // Sinusförmiges Signal generieren (256 Samples @ 48kHz ≈ 5.3ms)
        let mut samples = Vec::with_capacity(512);
        for i in 0..256 {
            let t = i as f32 / 256.0;
            let sample_l = amplitude * (2.0 * std::f32::consts::PI * 440.0 * t).sin();
            let sample_r = amplitude * 0.8 * (2.0 * std::f32::consts::PI * 880.0 * t).sin();
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
