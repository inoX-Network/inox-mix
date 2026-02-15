// Modul: audio/cpal_capture — Echtes Audio-Capture via CPAL
//
// Phase 2d: CPAL-Integration für Production-Ready Audio
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream, StreamConfig};
use log::{error, info};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// Audio-Sample (32-bit float, Stereo)
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioSample {
    pub left: f32,
    pub right: f32,
}

/// CPAL Audio-Capture Manager
///
/// Verwaltet echte Audio-Input-Streams von Hardware-Devices via CPAL
pub struct CpalCaptureManager {
    /// Aktive Streams (Device-Name → Stream-Handle)
    streams: Arc<Mutex<HashMap<String, CpalStreamHandle>>>,
    /// CPAL Host (Audio-System)
    host: cpal::Host,
}

/// Handle für einen CPAL-Stream
struct CpalStreamHandle {
    /// Device-Name
    device_name: String,
    /// Ring-Buffer für Audio-Daten (VecDeque ist einfacher als ringbuf)
    buffer: Arc<Mutex<VecDeque<AudioSample>>>,
    /// CPAL Stream (muss am Leben bleiben!)
    _stream: Stream,
}

impl CpalCaptureManager {
    /// Neuen CPAL Capture-Manager erstellen
    pub fn new() -> Result<Self, String> {
        info!("🎤 Initialisiere CPAL Audio-Capture...");

        // Standard CPAL Host (nutzt PipeWire auf Linux)
        let host = cpal::default_host();

        info!("✅ CPAL Host: {}", host.id().name());

        Ok(Self {
            streams: Arc::new(Mutex::new(HashMap::new())),
            host,
        })
    }

    /// Alle verfügbaren Input-Devices auflisten
    pub fn list_input_devices(&self) -> Result<Vec<String>, String> {
        let devices: Vec<String> = self
            .host
            .input_devices()
            .map_err(|e| format!("Fehler beim Auflisten der Devices: {}", e))?
            .filter_map(|device| device.name().ok())
            .collect();

        info!("📋 CPAL Input-Devices: {}", devices.len());
        for (i, name) in devices.iter().enumerate() {
            info!("  {}. {}", i + 1, name);
        }

        Ok(devices)
    }

    /// Default Input-Device holen
    pub fn get_default_input_device(&self) -> Result<Device, String> {
        self.host
            .default_input_device()
            .ok_or_else(|| "Kein Standard-Input-Device gefunden".to_string())
    }

    /// Input-Device nach Namen finden
    pub fn find_input_device(&self, name: &str) -> Result<Device, String> {
        self.host
            .input_devices()
            .map_err(|e| format!("Fehler beim Suchen des Devices: {}", e))?
            .find(|device| {
                device
                    .name()
                    .map(|n| n.to_lowercase().contains(&name.to_lowercase()))
                    .unwrap_or(false)
            })
            .ok_or_else(|| format!("Device '{}' nicht gefunden", name))
    }

    /// Audio-Capture für ein Device starten
    ///
    /// # Argumente
    /// * `device` - CPAL Input-Device
    /// * `stream_id` - Logische Stream-ID (z.B. "hw-mic-1")
    ///
    /// # Returns
    /// Ring-Buffer Arc für Audio-Daten
    pub fn start_capture(
        &mut self,
        device: Device,
        stream_id: &str,
    ) -> Result<Arc<Mutex<VecDeque<AudioSample>>>, String> {
        let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());

        info!(
            "🎙️  Starte Audio-Capture: {} (Device: {})",
            stream_id, device_name
        );

        // Prüfen ob bereits ein Stream existiert
        {
            let streams = self.streams.lock().unwrap();
            if streams.contains_key(stream_id) {
                return Err(format!("Stream {} läuft bereits", stream_id));
            }
        }

        // Device-Config abfragen
        let config = device
            .default_input_config()
            .map_err(|e| format!("Fehler beim Abrufen der Device-Config: {}", e))?;

        info!(
            "  📊 Format: {:?}, Sample-Rate: {}Hz, Kanäle: {}",
            config.sample_format(),
            config.sample_rate().0,
            config.channels()
        );

        // Ring-Buffer erstellen (2048 Samples ≈ 42ms @ 48kHz)
        let buffer = Arc::new(Mutex::new(VecDeque::with_capacity(2048)));
        let buffer_clone = Arc::clone(&buffer);

        // Audio-Stream basierend auf Sample-Format erstellen
        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                self.build_input_stream_f32(&device, config.into(), buffer_clone)?
            }
            SampleFormat::I16 => {
                self.build_input_stream_i16(&device, config.into(), buffer_clone)?
            }
            SampleFormat::U16 => {
                return Err("U16 Sample-Format nicht unterstützt".to_string());
            }
            _ => {
                return Err(format!(
                    "Sample-Format {:?} nicht unterstützt",
                    config.sample_format()
                ));
            }
        };

        // Stream starten
        stream
            .play()
            .map_err(|e| format!("Fehler beim Starten des Streams: {}", e))?;

        info!("✅ Audio-Capture gestartet: {}", stream_id);

        // Handle speichern
        let handle = CpalStreamHandle {
            device_name: device_name.clone(),
            buffer: Arc::clone(&buffer),
            _stream: stream,
        };

        self.streams
            .lock()
            .unwrap()
            .insert(stream_id.to_string(), handle);

        Ok(buffer)
    }

    /// Input-Stream für F32-Samples erstellen
    fn build_input_stream_f32(
        &self,
        device: &Device,
        config: StreamConfig,
        buffer: Arc<Mutex<VecDeque<AudioSample>>>,
    ) -> Result<Stream, String> {
        let channels = config.channels as usize;

        device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut buf = buffer.lock().unwrap();

                    // Audio-Daten in Ring-Buffer schreiben
                    // CPAL liefert interleaved samples: L, R, L, R, ...
                    for chunk in data.chunks_exact(channels) {
                        let sample = if channels >= 2 {
                            AudioSample {
                                left: chunk[0],
                                right: chunk[1],
                            }
                        } else {
                            // Mono → Beide Kanäle gleich
                            AudioSample {
                                left: chunk[0],
                                right: chunk[0],
                            }
                        };

                        // VecDeque als Ring-Buffer: Alte Samples entfernen wenn voll
                        if buf.len() >= 2048 {
                            buf.pop_front();
                        }
                        buf.push_back(sample);
                    }
                },
                |err| {
                    error!("CPAL Stream-Fehler: {}", err);
                },
                None, // Timeout
            )
            .map_err(|e| format!("Fehler beim Erstellen des Streams: {}", e))
    }

    /// Input-Stream für I16-Samples erstellen
    fn build_input_stream_i16(
        &self,
        device: &Device,
        config: StreamConfig,
        buffer: Arc<Mutex<VecDeque<AudioSample>>>,
    ) -> Result<Stream, String> {
        let channels = config.channels as usize;

        device
            .build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let mut buf = buffer.lock().unwrap();

                    // I16 → F32 Konvertierung
                    for chunk in data.chunks_exact(channels) {
                        let sample = if channels >= 2 {
                            AudioSample {
                                left: i16_to_f32(chunk[0]),
                                right: i16_to_f32(chunk[1]),
                            }
                        } else {
                            let mono = i16_to_f32(chunk[0]);
                            AudioSample {
                                left: mono,
                                right: mono,
                            }
                        };

                        if buf.len() >= 2048 {
                            buf.pop_front();
                        }
                        buf.push_back(sample);
                    }
                },
                |err| {
                    error!("CPAL Stream-Fehler: {}", err);
                },
                None,
            )
            .map_err(|e| format!("Fehler beim Erstellen des Streams: {}", e))
    }

    /// Audio-Capture stoppen
    pub fn stop_capture(&mut self, stream_id: &str) -> Result<(), String> {
        let mut streams = self.streams.lock().unwrap();
        if streams.remove(stream_id).is_some() {
            info!("🛑 Audio-Capture gestoppt: {}", stream_id);
            Ok(())
        } else {
            Err(format!("Stream {} nicht gefunden", stream_id))
        }
    }

    /// Alle Streams auflisten
    pub fn list_active_streams(&self) -> Vec<String> {
        let streams = self.streams.lock().unwrap();
        streams.keys().cloned().collect()
    }

    /// Shutdown
    pub fn shutdown(&mut self) {
        info!("🔌 CPAL Capture-Manager: Shutdown...");
        self.streams.lock().unwrap().clear();
    }
}

impl Drop for CpalCaptureManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// I16 Sample zu F32 konvertieren (normalized -1.0 bis 1.0)
#[inline]
fn i16_to_f32(sample: i16) -> f32 {
    sample as f32 / 32768.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpal_manager_creation() {
        let manager = CpalCaptureManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_i16_to_f32_conversion() {
        assert_eq!(i16_to_f32(0), 0.0);
        assert!((i16_to_f32(32767) - 1.0).abs() < 0.001);
        assert!((i16_to_f32(-32768) - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_audio_sample_default() {
        let sample = AudioSample::default();
        assert_eq!(sample.left, 0.0);
        assert_eq!(sample.right, 0.0);
    }

    #[test]
    #[ignore] // Benötigt echtes Audio-Device
    fn test_list_input_devices() {
        let manager = CpalCaptureManager::new().unwrap();
        let devices = manager.list_input_devices();
        assert!(devices.is_ok());
        println!("Devices: {:?}", devices.unwrap());
    }

    #[test]
    #[ignore] // Benötigt echtes Audio-Device
    fn test_get_default_device() {
        let manager = CpalCaptureManager::new().unwrap();
        let device = manager.get_default_input_device();
        assert!(device.is_ok());
    }
}
