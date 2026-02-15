// Modul: audio/capture — PipeWire Audio-Capture für Echtzeit-Metering
use ringbuf::{HeapRb, traits::*};
use log::{info, warn};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Audio-Buffer-Größe (2048 Samples = ~42ms @ 48kHz)
const AUDIO_BUFFER_SIZE: usize = 2048;

/// Maximale Anzahl paralleler Streams
const MAX_STREAMS: usize = 16;

/// Audio-Sample (32-bit float, Stereo)
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioSample {
    pub left: f32,
    pub right: f32,
}

/// Audio-Stream State
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamState {
    /// Stream ist inaktiv
    Inactive,
    /// Stream verbindet sich
    Connecting,
    /// Stream ist aktiv und liefert Audio-Daten
    Active,
    /// Stream hat einen Fehler
    Error(String),
}

/// PipeWire Audio-Capture Manager
///
/// Verwaltet mehrere Audio-Streams von verschiedenen PipeWire-Nodes
/// und stellt die Audio-Daten über Ring-Buffer bereit.
///
/// Phase 2b: Placeholder-Implementierung
pub struct AudioCaptureManager {
    /// Aktive Audio-Streams (Node-ID → Stream-Handle)
    streams: Arc<Mutex<HashMap<String, AudioStreamHandle>>>,
}

/// Handle für einen einzelnen Audio-Stream
struct AudioStreamHandle {
    /// Stream-ID (z.B. "hw-mic-1")
    stream_id: String,
    /// PipeWire-Node-ID (numerisch)
    node_id: u32,
    /// Ring-Buffer (für Producer/Consumer Zugriff)
    buffer: Arc<Mutex<HeapRb<AudioSample>>>,
    /// Stream-Status
    state: Arc<Mutex<StreamState>>,
}

impl AudioCaptureManager {
    /// Neuen Capture-Manager erstellen
    pub fn new() -> Self {
        Self {
            streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Audio-Capture für einen Node starten
    ///
    /// # Argumente
    /// * `stream_id` - Logische Stream-ID (z.B. "hw-mic-1", "virt-browser")
    /// * `node_id` - PipeWire-Node-ID (aus Node-Discovery)
    ///
    /// # Returns
    /// Ok() wenn erfolgreich gestartet
    pub fn start_capture(
        &mut self,
        stream_id: &str,
        node_id: u32,
    ) -> Result<(), String> {
        info!("Starte Audio-Capture: {} (Node {})", stream_id, node_id);

        // Prüfen ob bereits ein Stream für diese ID existiert
        {
            let streams = self.streams.lock().unwrap();
            if streams.contains_key(stream_id) {
                warn!("Stream {} existiert bereits", stream_id);
                return Err(format!("Stream {} läuft bereits", stream_id));
            }
        }

        // Prüfen ob Limit erreicht
        {
            let streams = self.streams.lock().unwrap();
            if streams.len() >= MAX_STREAMS {
                return Err(format!("Maximum {} Streams erreicht", MAX_STREAMS));
            }
        }

        // Ring-Buffer erstellen (AUDIO_BUFFER_SIZE Samples)
        let buffer = Arc::new(Mutex::new(HeapRb::<AudioSample>::new(AUDIO_BUFFER_SIZE)));

        // Phase 2b: PipeWire Stream-Implementierung
        // TODO: Echte Stream-Integration mit pipewire-rs API
        //
        // Benötigte Schritte:
        // 1. Stream::new() mit Core erstellen
        // 2. StreamListener registrieren mit .process() Callback
        // 3. Im Callback: Audio-Daten aus PipeWire-Buffer lesen
        // 4. Samples in Ring-Buffer schreiben
        // 5. Stream.connect() mit node_id aufrufen
        //
        // Referenz: https://docs.pipewire.org/page_tutorial5.html
        //
        // Für jetzt: Placeholder-Status (wird in Phase 2c implementiert)
        let state = Arc::new(Mutex::new(StreamState::Connecting));

        info!(
            "Stream-Placeholder erstellt: {} (Node {})",
            stream_id, node_id
        );
        info!(
            "⚠️  Phase 2b TODO: Echte PipeWire Stream-Integration implementieren"
        );

        let handle = AudioStreamHandle {
            stream_id: stream_id.to_string(),
            node_id,
            buffer,
            state,
        };

        // Stream-Handle speichern
        {
            let mut streams = self.streams.lock().unwrap();
            streams.insert(stream_id.to_string(), handle);
        }

        info!("Audio-Capture gestartet: {}", stream_id);
        Ok(())
    }

    /// Audio-Samples aus einem Stream lesen
    ///
    /// Liest verfügbare Audio-Samples aus dem Ring-Buffer eines Streams
    ///
    /// TODO Phase 2c: Ring-Buffer API korrekt nutzen
    pub fn read_samples(&self, _stream_id: &str, _max_samples: usize) -> Vec<f32> {
        // Placeholder: Wird in Phase 2c mit echtem PipeWire-Stream implementiert
        Vec::new()
    }

    /// Audio-Capture für einen Node stoppen
    pub fn stop_capture(&mut self, stream_id: &str) -> Result<(), String> {
        info!("Stoppe Audio-Capture: {}", stream_id);

        let mut streams = self.streams.lock().unwrap();
        if let Some(handle) = streams.remove(stream_id) {
            // Stream-Status auf Inactive setzen
            *handle.state.lock().unwrap() = StreamState::Inactive;
            info!("Audio-Capture gestoppt: {}", stream_id);
            Ok(())
        } else {
            Err(format!("Stream {} nicht gefunden", stream_id))
        }
    }

    /// Status eines Streams abfragen
    pub fn get_stream_state(&self, stream_id: &str) -> Option<StreamState> {
        let streams = self.streams.lock().unwrap();
        streams.get(stream_id).map(|h| h.state.lock().unwrap().clone())
    }

    /// Alle aktiven Streams auflisten
    pub fn list_active_streams(&self) -> Vec<String> {
        let streams = self.streams.lock().unwrap();
        streams.keys().cloned().collect()
    }

    /// Alle Streams stoppen und Ressourcen freigeben
    pub fn shutdown(&mut self) {
        info!("AudioCaptureManager: Shutdown...");

        // Alle Streams stoppen
        let stream_ids: Vec<String> = {
            let streams = self.streams.lock().unwrap();
            streams.keys().cloned().collect()
        };

        for stream_id in stream_ids {
            let _ = self.stop_capture(&stream_id);
        }

        info!("AudioCaptureManager: Shutdown abgeschlossen");
    }
}

impl Drop for AudioCaptureManager {
    fn drop(&mut self) {
        self.shutdown();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_capture_manager_creation() {
        let manager = AudioCaptureManager::new();
        assert_eq!(manager.list_active_streams().len(), 0);
    }

    #[test]
    fn test_audio_sample_default() {
        let sample = AudioSample::default();
        assert_eq!(sample.left, 0.0);
        assert_eq!(sample.right, 0.0);
    }

    #[test]
    fn test_stream_state_serialization() {
        let state = StreamState::Active;
        let json = serde_json::to_string(&state);
        assert!(json.is_ok());
    }

    #[test]
    fn test_read_samples_empty_stream() {
        let manager = AudioCaptureManager::new();
        let samples = manager.read_samples("nonexistent", 10);
        assert_eq!(samples.len(), 0);
    }

}
